extern crate diesel;
extern crate dotenv;

use crate::models::{
    Balances, BalancesInsert, History, HistoryInsert, Inscriptions, InscriptionsInsert, Status,
    Tracker, TrackerInsert,
};
use crate::schema::{balances, history, inscriptions, status, tracker};
use bigdecimal::BigDecimal;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::env;

pub struct Database {
    pub connection: PgConnection,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

impl Database {
    pub fn new() -> Result<Database, Error> {
        let connection = establish_connection();
        Ok(Database { connection })
    }

    pub fn get_status(&mut self, key: String) -> QueryResult<Status> {
        status::table
            .filter(status::key.eq(&key))
            .first(&mut self.connection)
    }

    pub fn update_status(&mut self, key: String, value: String) -> QueryResult<Status> {
        diesel::update(status::table)
            .filter(status::key.eq(&key))
            .set(status::value.eq(&value))
            .get_result(&mut self.connection)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_inscription(
        &mut self,
        tx_id: String,
        address: String,
        inscription_id: String,
        inscription_num: i64,
        height: i64,
        output: String,
        timestamp: i64,
        ticker: String,
        action: String,
        supply: Option<BigDecimal>,
        limit_mint: Option<BigDecimal>,
        decimal: Option<i32>,
        amount: Option<BigDecimal>,
        value: i64,
    ) -> QueryResult<Inscriptions> {
        let new_inscription = InscriptionsInsert {
            genesis_tx_id: tx_id,
            genesis_address: address,
            ticker,
            action,
            supply,
            limit_mint,
            decimal,
            amount,
            value,
            inscription_id,
            inscription_num,
            output,
            height,
            timestamp,
        };

        diesel::insert_into(inscriptions::table)
            .values(&new_inscription)
            .returning(Inscriptions::as_returning())
            .get_result(&mut self.connection)
    }

    #[allow(unused)]
    pub fn get_inscription(&mut self, id: i32) -> QueryResult<Inscriptions> {
        inscriptions::table
            .filter(inscriptions::id.eq(&id))
            .first(&mut self.connection)
    }

    pub fn get_inscription_by_output(&mut self, output: String) -> QueryResult<Inscriptions> {
        inscriptions::table
            .filter(inscriptions::output.eq(&output))
            .first(&mut self.connection)
    }

    pub fn update_inscription_spent(
        &mut self,
        id: i32,
        address_sender: String,
        address_receiver: String,
        spent_tx: String,
        spent_offset: i64,
        spent_height: i64,
    ) -> QueryResult<Inscriptions> {
        diesel::update(inscriptions::table)
            .filter(inscriptions::id.eq(&id))
            .set((
                inscriptions::address_sender.eq(&address_sender),
                inscriptions::address_receiver.eq(&address_receiver),
                inscriptions::spent_tx.eq(&spent_tx),
                inscriptions::spent.eq(true),
                inscriptions::spent_offset.eq(&spent_offset),
                inscriptions::spent_height.eq(&spent_height),
            ))
            .get_result(&mut self.connection)
    }

    pub fn get_latest_inscription(&mut self) -> QueryResult<Inscriptions> {
        inscriptions::table
            .order(inscriptions::id.desc())
            .first(&mut self.connection)
    }

    pub fn create_tracker(
        &mut self,
        ticker: String,
        inscription_id: String,
        deploy_inscription_num: i64,
        supply: BigDecimal,
        limit: BigDecimal,
        decimals: i32,
    ) -> Tracker {
        let new_tracker = TrackerInsert {
            deploy_inscription_num,
            deploy_inscription: inscription_id,
            ticker,
            supply,
            limit_mint: limit,
            decimals,
        };

        diesel::insert_into(tracker::table)
            .values(&new_tracker)
            .returning(Tracker::as_returning())
            .get_result(&mut self.connection)
            .expect("Error saving new tracker")
    }

    pub fn get_tracker(&mut self, ticker: String) -> QueryResult<Tracker> {
        tracker::table
            .filter(tracker::ticker.eq(&ticker))
            .first(&mut self.connection)
    }

    pub fn update_tracker_minted(
        &mut self,
        id: i32,
        supply_minted: BigDecimal,
    ) -> QueryResult<Tracker> {
        diesel::update(tracker::table)
            .filter(tracker::id.eq(&id))
            .set(tracker::supply_minted.eq(&supply_minted))
            .get_result(&mut self.connection)
    }

    pub fn update_tracker_holders(&mut self, id: i32, holders: i64) -> QueryResult<Tracker> {
        diesel::update(tracker::table)
            .filter(tracker::id.eq(&id))
            .set(tracker::holders.eq(&holders))
            .get_result(&mut self.connection)
    }

    pub fn update_tracker_transactions(
        &mut self,
        id: i32,
        transactions: i64,
    ) -> QueryResult<Tracker> {
        diesel::update(tracker::table)
            .filter(tracker::id.eq(&id))
            .set(tracker::transactions.eq(&transactions))
            .get_result(&mut self.connection)
    }

    pub fn update_tracker_mint_start(&mut self, id: i32, mint_start: i64) -> QueryResult<Tracker> {
        diesel::update(tracker::table)
            .filter(tracker::id.eq(&id))
            .set(tracker::inscription_mint_start.eq(&mint_start))
            .get_result(&mut self.connection)
    }

    pub fn update_tracker_mint_end(&mut self, id: i32, mint_end: i64) -> QueryResult<Tracker> {
        diesel::update(tracker::table)
            .filter(tracker::id.eq(&id))
            .set(tracker::inscription_mint_end.eq(&mint_end))
            .get_result(&mut self.connection)
    }

    pub fn create_balance(&mut self, address: String, ticker: String) -> QueryResult<Balances> {
        let new_balance = BalancesInsert { address, ticker };

        diesel::insert_into(balances::table)
            .values(&new_balance)
            .returning(Balances::as_returning())
            .get_result(&mut self.connection)
    }

    pub fn get_balance(&mut self, address: String, ticker: String) -> QueryResult<Balances> {
        balances::table
            .filter(
                balances::ticker
                    .eq(&ticker)
                    .and(balances::address.eq(&address)),
            )
            .first(&mut self.connection)
    }

    pub fn update_balance(
        &mut self,
        address: String,
        ticker: String,
        balance: BigDecimal,
        transfer_balance: BigDecimal,
    ) -> QueryResult<Balances> {
        diesel::update(balances::table)
            .filter(
                balances::address
                    .eq(&address)
                    .and(balances::ticker.eq(&ticker)),
            )
            .set((
                balances::balance.eq(&balance),
                balances::transfer_balance.eq(&transfer_balance),
            ))
            .get_result(&mut self.connection)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_history(
        &mut self,
        address_sender: String,
        address_receiver: String,
        amount: BigDecimal,
        ticker: String,
        action: String,
        invalid: bool,
        inscription_id: String,
        inscription_num: i64,
        tx_id: String,
        height: i64,
        timestamp: i64,
    ) -> QueryResult<History> {
        let new_history = HistoryInsert {
            address_sender: Some(address_sender),
            address_receiver: Some(address_receiver),
            amount,
            ticker,
            action,
            invalid,
            inscription_id,
            inscription_num,
            tx_id,
            height,
            timestamp,
        };

        diesel::insert_into(history::table)
            .values(&new_history)
            .returning(History::as_returning())
            .get_result(&mut self.connection)
    }
}
