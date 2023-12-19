use bigdecimal::BigDecimal;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Ord20 {
    pub p: String,
    pub op: String,
    pub tick: String,
    pub max: Option<BigDecimal>,
    pub lim: Option<BigDecimal>,
    pub amt: Option<BigDecimal>,
    pub dec: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::status)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Status {
    pub id: i32,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::inscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InscriptionsInsert {
    pub genesis_tx_id: String,
    pub genesis_address: String,
    pub ticker: String,
    pub action: String,
    pub supply: Option<BigDecimal>,
    pub limit_mint: Option<BigDecimal>,
    pub decimal: Option<i32>,
    pub amount: Option<BigDecimal>,
    pub inscription_id: String,
    pub inscription_num: i64,
    pub output: String,
    pub value: i64,
    pub height: i64,
    pub timestamp: i64,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::inscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct InscriptionsTransferInsert {
    pub address_sender: Option<String>,
    pub address_receiver: Option<String>,
    pub spent: Option<bool>,
    pub spent_tx: Option<String>,
    pub spent_offset: Option<i64>,
    pub spent_height: Option<i64>,
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = crate::schema::inscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Inscriptions {
    pub id: i32,
    pub genesis_tx_id: String,
    pub genesis_address: String,
    pub address_sender: Option<String>,
    pub address_receiver: Option<String>,
    pub ticker: String,
    pub action: String,
    pub supply: Option<BigDecimal>,
    pub limit_mint: Option<BigDecimal>,
    pub decimal: Option<i32>,
    pub amount: Option<BigDecimal>,
    pub inscription_id: String,
    pub inscription_num: i64,
    pub height: i64,
    pub timestamp: i64,
    pub output: String,
    pub value: Option<i64>,
    pub valid: Option<bool>,
    pub spent: Option<bool>,
    pub spent_tx: Option<String>,
    pub spent_offset: Option<i64>,
    pub spent_height: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tracker)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TrackerInsert {
    pub deploy_inscription_num: i64,
    pub deploy_inscription: String,
    pub ticker: String,
    pub supply: BigDecimal,
    pub limit_mint: BigDecimal,
    pub decimals: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tracker)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Tracker {
    pub id: i32,
    pub deploy_inscription_num: i64,
    pub deploy_inscription: String,
    pub ticker: String,
    pub supply: BigDecimal,
    pub supply_minted: BigDecimal,
    pub limit_mint: BigDecimal,
    pub decimals: i32,
    pub holders: i64,
    pub transactions: i64,
    pub inscription_mint_start: Option<i64>,
    pub inscription_mint_end: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BalancesInsert {
    pub address: String,
    pub ticker: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::balances)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Balances {
    pub id: i32,
    pub address: String,
    pub ticker: String,
    pub balance: BigDecimal,
    pub transfer_balance: BigDecimal,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HistoryInsert {
    pub address_sender: Option<String>,
    pub address_receiver: Option<String>,
    pub amount: BigDecimal,
    pub ticker: String,
    pub action: String,
    pub invalid: bool,
    pub inscription_id: String,
    pub inscription_num: i64,
    pub tx_id: String,
    pub height: i64,
    pub timestamp: i64,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::history)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct History {
    pub id: i32,
    pub address_sender: Option<String>,
    pub address_receiver: Option<String>,
    pub amount: BigDecimal,
    pub ticker: String,
    pub action: String,
    pub invalid: bool,
    pub inscription_id: String,
    pub inscription_num: i64,
    pub tx_id: String,
    pub height: i64,
    pub timestamp: i64,
}
