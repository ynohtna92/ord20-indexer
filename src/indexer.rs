use crate::database::Database;
use crate::models::{Inscriptions, Ord20};
use crate::ordinals::{Block, Inscription, Ordinals};
use crate::util::{bigdecimal_fractional_count, string_to_timestamp};
use crate::SHUTTING_DOWN;
use bigdecimal::{BigDecimal, Zero};
use hex::decode;
use lazy_static::lazy_static;
use std::convert::Into;
use std::env;
use std::string::ToString;
use std::sync::atomic::Ordering;
use std::sync::{mpsc, Arc};
use std::time::Instant;
use tokio::sync::Semaphore;

lazy_static! {
    static ref MAX_CONCURRENT_REQUESTS: usize = {
        let max_concurrent_requests_str =
            env::var("MAX_CONCURRENT_REQUESTS").unwrap_or("10".to_string());
        max_concurrent_requests_str
            .parse::<usize>()
            .expect("MAX_CONCURRENT_REQUESTS must be a positive integer")
    };
}

pub struct Indexer {
    pub ordinals: Ordinals,
    pub database: Database,
    pub meta_protocol: String,
}

impl Indexer {
    pub(crate) async fn get_blocks(&mut self, target_block: i32) {
        let semaphore = Arc::new(Semaphore::new(*MAX_CONCURRENT_REQUESTS));
        let (tx, rx) = mpsc::sync_channel::<_>(*MAX_CONCURRENT_REQUESTS * 2);
        let status = self
            .database
            .get_status("last_height".to_string())
            .unwrap()
            .value;
        let ordinals = self.ordinals.clone();
        tokio::spawn(async move {
            let mut last_height = status.unwrap().parse::<i32>().unwrap();

            log::info!(
                "Last Block Processed {}, Target Block {}",
                last_height,
                target_block
            );

            while last_height < target_block {
                let next_height = last_height + 1;
                let mut ordinals = ordinals.clone();
                let semaphore = Arc::clone(&semaphore);
                let block_request = tokio::spawn(async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    ordinals.get_block(next_height).await
                });

                last_height = next_height;

                tx.send(block_request)
                    .expect("Failed to send block to channel");

                if SHUTTING_DOWN.load(Ordering::Relaxed) {
                    break;
                }
            }
        });

        while let Ok(block_future) = rx.recv() {
            match block_future.await {
                Ok(block) => {
                    let start_time = Instant::now();
                    let processed = self.process_block(block.as_ref().unwrap()).await;
                    let elapsed_time = start_time.elapsed();
                    log::info!(
                        "Block {}/{}, Timestamp: {}, Txs: {}, Inscriptions: {}, Time: {:?}",
                        block.as_ref().unwrap().height,
                        target_block,
                        block.as_ref().unwrap().timestamp,
                        block.as_ref().unwrap().transactions.len(),
                        processed,
                        elapsed_time
                    );
                    let _ = self.database.update_status(
                        "last_height".to_string(),
                        block.as_ref().unwrap().height.to_string(),
                    );
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            }
        }
    }

    pub(crate) async fn process_block(&mut self, block: &Block) -> i32 {
        let mut inscriptions_count = 0;
        let mut block_miner_address = "";
        for txs in &block.transactions {
            let address = if !txs.output_addresses.is_empty() {
                txs.output_addresses.first().unwrap()
            } else {
                ""
            };
            if block_miner_address.is_empty() {
                if txs.output_addresses.is_empty() {
                    log::debug!("Block coinbase sent to empty address");
                } else {
                    block_miner_address = txs.output_addresses.first().unwrap();
                    log::debug!("Block Miner Address: {}", block_miner_address);
                }
            }
            let mut fetched_up_to_index = -1;
            let mut input_offset = 0;
            for (index, (input, _input_value)) in txs.inputs.iter().enumerate() {
                // Check inputs for transfer inscription
                if let Ok(inscription) = self.database.get_inscription_by_output(input.to_string())
                {
                    if inscription.action.contains("transfer")
                        && !inscription.spent.unwrap_or_default()
                    {
                        if index > 0 {
                            for backfill_index in (fetched_up_to_index + 1) as usize..index {
                                let (backfill_input, _) = &txs.inputs[backfill_index];

                                // Fetch the input_value from the database for this backfill input
                                if let Ok(output) =
                                    self.ordinals.get_output(backfill_input.to_string()).await
                                {
                                    log::info!(
                                        "Get output for index {}:  {:?}",
                                        backfill_input,
                                        output
                                    );
                                    input_offset += output.value;
                                    // Update fetched_up_to_index since we've now fetched this input_value
                                    fetched_up_to_index = backfill_index as isize;
                                } else {
                                    log::error!(
                                        "Failed to get output value for input: {}",
                                        backfill_input.to_string()
                                    );
                                }
                            }
                        }

                        let vout =
                            Indexer::calculate_ordinal_position(index, input_offset, &txs.outputs);
                        let address_receiver = if vout > txs.outputs.len() - 1 {
                            // Invalid output - Return balance to sender address
                            inscription.genesis_address.as_str()
                        } else {
                            txs.output_addresses.get(vout).unwrap()
                        };
                        if let Ok(transfer_inscription) = self.database.update_inscription_spent(
                            inscription.id,
                            inscription.genesis_address.clone(),
                            address_receiver.to_string(),
                            txs.transaction.clone(),
                            0,
                            block.height as i64,
                            string_to_timestamp(block.timestamp.as_str()).unwrap_or(0),
                        ) {
                            if transfer_inscription.valid.unwrap_or_default() {
                                self.process_inscription_transfer(&transfer_inscription);
                            }
                        }
                    }
                }
            }
            if !txs.inscriptions.is_empty() && address.is_empty() {
                log::warn!("Empty address on tx {}", txs.transaction);
            }
            for tx_inscription in &txs.inscriptions {
                let inscription = Inscription {
                    number: tx_inscription.inscription_number,
                    offset: 0,
                    genesis_fee: 0,
                    genesis_height: block.height as i64,
                    genesis_transaction: txs.transaction.clone(),
                    inscription_id: tx_inscription.inscription_id.clone(),
                    output: tx_inscription.inscription_id.replace('i', ":"),
                    location: "".to_string(),
                    address: "".to_string(),
                    genesis_address: address.to_string(),
                    content_type: tx_inscription.content_type.as_ref().map(|s| {
                        decode(s.as_bytes())
                            .map(|decoded| String::from_utf8_lossy(&decoded).into_owned())
                            .unwrap_or_else(|_| String::new())
                    }),
                    timestamp: block.timestamp.clone(),
                };
                if inscription.number > 0
                    && inscription.content_type.as_ref().is_some_and(|ct| {
                        ct.contains("text/plain") || ct.contains("application/json")
                    })
                {
                    if let Some(inscription) = self.add_inscription(
                        inscription,
                        decode(tx_inscription.content.clone().unwrap())
                            .map(|s| String::from_utf8_lossy(s.as_slice()).into_owned())
                            .unwrap(),
                    ) {
                        log::debug!("Process Inscription: {:?}", inscription);
                        inscriptions_count += 1;
                        self.process_inscription(&inscription);
                    }
                }
            }
        }
        inscriptions_count
    }

    pub(crate) fn calculate_ordinal_position(
        input_index: usize,
        ordinal_offset: u64,
        outputs: &Vec<(String, u64)>,
    ) -> usize {
        let mut output_index = 0;
        let mut output_count = 0;

        // Skip check if ordinal input is first input in transaction
        // Assumptions: ordinal inscribed at offset 0 of input is on output 0
        if input_index == 0 {
            return output_index;
        }

        for (_, output) in outputs {
            output_count += output;
            if output_count > ordinal_offset {
                break;
            }
            output_index += 1;
        }

        output_index
    }

    pub(crate) fn add_inscription(
        &mut self,
        inscription: Inscription,
        content: String,
    ) -> Option<Inscriptions> {
        if let Ok(ord20) = serde_json::from_str::<Ord20>(&content) {
            if !ord20.p.contains(self.meta_protocol.as_str()) {
                return None;
            }
            let inscription_id = if inscription.inscription_id.is_empty() {
                format!("{}i{}", inscription.genesis_transaction, inscription.output)
            } else {
                inscription.inscription_id
            };
            // Add to inscription database
            return self
                .database
                .create_inscription(
                    inscription.genesis_transaction,
                    inscription.genesis_address,
                    inscription_id,
                    inscription.number,
                    inscription.genesis_height,
                    inscription.output,
                    string_to_timestamp(inscription.timestamp.as_str()).unwrap(),
                    ord20.tick.to_lowercase(),
                    ord20.op,
                    ord20.max,
                    ord20.lim,
                    ord20.dec,
                    ord20.amt,
                    0,
                )
                .ok();
        }
        log::info!("Error processing content {}", content);
        None
    }

    pub(crate) fn process_inscription(&mut self, inscription: &Inscriptions) {
        let ticker = self.database.get_tracker(inscription.ticker.clone());

        if &inscription.action == "deploy" {
            if ticker.is_err() {
                let decimal = inscription.decimal.unwrap_or(18);

                if !(0..=18).contains(&decimal) {
                    return;
                }

                #[allow(clippy::needless_as_bytes)]
                if inscription.ticker.as_bytes().len() != 4 {
                    return;
                }

                if inscription.supply.clone().unwrap_or_default() <= BigDecimal::zero()
                    || inscription
                        .supply
                        .clone()
                        .unwrap_or(BigDecimal::from(i64::MAX - 1))
                        > i64::MAX.into()
                    || inscription.limit_mint.clone().unwrap_or_default() < BigDecimal::zero()
                {
                    return;
                }

                self.database.create_tracker(
                    inscription.ticker.clone(),
                    inscription.inscription_id.clone(),
                    inscription.inscription_num,
                    inscription.supply.clone().unwrap_or_default(),
                    inscription.limit_mint.clone().unwrap_or_default(),
                    decimal,
                );
            }
        } else if &inscription.action == "mint" {
            if let Ok(ticker) = ticker {
                let limit = ticker.limit_mint;

                let amount = inscription.amount.clone().unwrap_or_default();

                if amount <= BigDecimal::zero()
                    || bigdecimal_fractional_count(amount.clone()) > ticker.decimals as u32
                {
                    return;
                }

                if (limit == BigDecimal::zero() || limit >= amount)
                    && ticker.supply != ticker.supply_minted.clone()
                {
                    if ticker.supply_minted.clone() == BigDecimal::zero() {
                        let _ = self
                            .database
                            .update_tracker_mint_start(ticker.id, inscription.inscription_num);
                    }
                    if ticker.supply_minted.clone() + amount.clone() >= ticker.supply {
                        let _ = self
                            .database
                            .update_tracker_mint_end(ticker.id, inscription.inscription_num);
                    }

                    let mut mint_balance = amount.clone();
                    let mut new_supply = ticker.supply_minted + mint_balance.clone();

                    if new_supply > ticker.supply {
                        mint_balance = amount.clone() - (new_supply - ticker.supply.clone());
                        new_supply = ticker.supply;
                    }

                    let _ = self.database.update_tracker_minted(ticker.id, new_supply);

                    let mut transfer_balance = BigDecimal::zero();

                    if let Ok(account) = self
                        .database
                        .get_balance(inscription.genesis_address.clone(), ticker.ticker.clone())
                    {
                        mint_balance = account.balance + mint_balance.clone();
                        transfer_balance = account.transfer_balance + transfer_balance;
                    } else {
                        let _ = self.database.create_balance(
                            inscription.genesis_address.clone(),
                            ticker.ticker.clone(),
                        );
                        let _ = self
                            .database
                            .update_tracker_holders(ticker.id, ticker.holders + 1);
                    }

                    let _ = self.database.update_balance(
                        inscription.genesis_address.clone(),
                        ticker.ticker.clone(),
                        mint_balance,
                        transfer_balance,
                    );

                    let _ = self.database.create_history(
                        "".to_string(),
                        inscription.genesis_address.clone(),
                        inscription.amount.clone().unwrap(),
                        ticker.ticker.clone(),
                        inscription.action.clone(),
                        false,
                        inscription.inscription_id.clone(),
                        inscription.inscription_num,
                        inscription.genesis_tx_id.clone(),
                        inscription.height,
                        inscription.timestamp,
                    );
                }
            }
        } else if &inscription.action == "transfer" && inscription.amount.is_some() {
            if let Ok(ticker) = ticker {
                if let Ok(account) = self
                    .database
                    .get_balance(inscription.genesis_address.clone(), ticker.ticker.clone())
                {
                    let amount = inscription.amount.clone().unwrap();

                    if amount <= BigDecimal::zero()
                        || bigdecimal_fractional_count(amount.clone()) > ticker.decimals as u32
                    {
                        return;
                    }

                    let mut invalid = false;
                    if account.balance >= amount {
                        let new_balance = account.balance - amount.clone();
                        let transfer_balance = account.transfer_balance + amount.clone();

                        let _ = self.database.update_balance(
                            inscription.genesis_address.clone(),
                            ticker.ticker.clone(),
                            new_balance,
                            transfer_balance,
                        );
                    } else {
                        invalid = true;
                    }

                    let _ = self.database.create_history(
                        inscription.genesis_address.clone(),
                        "".to_string(),
                        inscription.amount.clone().unwrap(),
                        ticker.ticker.clone(),
                        inscription.action.clone(),
                        invalid,
                        inscription.inscription_id.clone(),
                        inscription.inscription_num,
                        inscription.genesis_tx_id.clone(),
                        inscription.height,
                        inscription.timestamp,
                    );

                    let _ = self
                        .database
                        .update_inscription_valid(inscription.id, !invalid);
                }
            }
        }
    }

    pub(crate) fn process_inscription_transfer(&mut self, inscription: &Inscriptions) {
        let amount = inscription.amount.clone().unwrap_or_default();

        let mut receiver_balance_current = BigDecimal::zero();
        let mut receiver_transfer_balance_current = BigDecimal::zero();

        let mut holders_change = 0;

        if let Ok(receiver_balance) = self.database.get_balance(
            inscription.address_receiver.clone().unwrap(),
            inscription.ticker.clone(),
        ) {
            if receiver_balance.balance.clone() == BigDecimal::zero()
                && receiver_balance.transfer_balance.clone() == BigDecimal::zero()
            {
                holders_change = 1;
            }

            receiver_balance_current = receiver_balance.balance;
            receiver_transfer_balance_current = receiver_balance.transfer_balance;
        } else {
            // If no existing entry exists then create a new one
            let _ = self.database.create_balance(
                inscription.address_receiver.clone().unwrap(),
                inscription.ticker.clone(),
            );
            holders_change = 1;
        }

        if let Ok(sender_balance) = self.database.get_balance(
            inscription.address_sender.clone().unwrap(),
            inscription.ticker.clone(),
        ) {
            let sender_transfer_balance_new = sender_balance.transfer_balance - amount.clone();
            let receiver_balance_new = receiver_balance_current + amount;

            if sender_balance.balance.clone() == BigDecimal::zero()
                && sender_transfer_balance_new.clone() == BigDecimal::zero()
            {
                holders_change -= 1;
            }

            if inscription
                .address_sender
                .clone()
                .unwrap()
                .eq(&inscription.address_receiver.clone().unwrap())
            {
                let _ = self.database.update_balance(
                    inscription.address_sender.clone().unwrap(),
                    inscription.ticker.clone(),
                    receiver_balance_new,
                    sender_transfer_balance_new,
                );
            } else {
                let _ = self.database.update_balance(
                    inscription.address_sender.clone().unwrap(),
                    inscription.ticker.clone(),
                    sender_balance.balance,
                    sender_transfer_balance_new,
                );
                let _ = self.database.update_balance(
                    inscription.address_receiver.clone().unwrap(),
                    inscription.ticker.clone(),
                    receiver_balance_new,
                    receiver_transfer_balance_current,
                );
            }

            if let Ok(tracker) = self.database.get_tracker(inscription.ticker.clone()) {
                let _ = self
                    .database
                    .update_tracker_holders(tracker.id, tracker.holders + holders_change);
                let _ = self
                    .database
                    .update_tracker_transactions(tracker.id, tracker.transactions + 1);
            }

            let _ = self.database.create_history(
                inscription.address_sender.clone().unwrap(),
                inscription.address_receiver.clone().unwrap(),
                inscription.amount.clone().unwrap(),
                inscription.ticker.clone(),
                "send".to_string(),
                false,
                inscription.inscription_id.clone(),
                inscription.inscription_num,
                inscription.genesis_tx_id.clone(),
                inscription.height,
                inscription.timestamp,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::indexer::Indexer;

    #[test]
    fn test_calculate_ordinal_position_01() {
        let ordinal_offset = 10;
        let outputs = vec![
            ("output1".to_string(), 10),
            ("output2".to_string(), 5),
            ("output3".to_string(), 20),
        ];
        assert_eq!(
            Indexer::calculate_ordinal_position(usize::MAX, ordinal_offset, &outputs),
            1
        );
    }

    #[test]
    fn test_calculate_ordinal_position_02() {
        let ordinal_offset = 0;
        let outputs = vec![
            ("output1".to_string(), 10),
            ("output2".to_string(), 5),
            ("output3".to_string(), 20),
        ];
        assert_eq!(
            Indexer::calculate_ordinal_position(usize::MAX, ordinal_offset, &outputs),
            0
        );
    }

    #[test]
    fn test_calculate_ordinal_position_03() {
        let ordinal_offset = 40;
        let outputs = vec![
            ("output1".to_string(), 10),
            ("output2".to_string(), 5),
            ("output3".to_string(), 20),
        ];
        assert_eq!(
            Indexer::calculate_ordinal_position(usize::MAX, ordinal_offset, &outputs),
            3
        );
    }
}
