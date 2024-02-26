// @generated automatically by Diesel CLI.

diesel::table! {
    balances (id) {
        id -> Int4,
        address -> Text,
        ticker -> Text,
        balance -> Numeric,
        transfer_balance -> Numeric,
    }
}

diesel::table! {
    history (id) {
        id -> Int4,
        address_sender -> Nullable<Text>,
        address_receiver -> Nullable<Text>,
        amount -> Numeric,
        ticker -> Text,
        action -> Text,
        invalid -> Bool,
        tx_id -> Text,
        inscription_id -> Text,
        inscription_num -> Int8,
        height -> Int8,
        timestamp -> Int8,
    }
}

diesel::table! {
    inscriptions (id) {
        id -> Int4,
        genesis_tx_id -> Text,
        genesis_address -> Text,
        address_sender -> Nullable<Text>,
        address_receiver -> Nullable<Text>,
        ticker -> Text,
        action -> Text,
        supply -> Nullable<Numeric>,
        limit_mint -> Nullable<Numeric>,
        decimal -> Nullable<Int4>,
        amount -> Nullable<Numeric>,
        inscription_id -> Text,
        inscription_num -> Int8,
        height -> Int8,
        timestamp -> Int8,
        output -> Text,
        value -> Nullable<Int8>,
        valid -> Nullable<Bool>,
        spent -> Nullable<Bool>,
        spent_tx -> Nullable<Text>,
        spent_offset -> Nullable<Int8>,
        spent_height -> Nullable<Int8>,
        spent_timestamp -> Nullable<Int8>,
    }
}

diesel::table! {
    status (id) {
        id -> Int4,
        key -> Text,
        value -> Nullable<Text>,
    }
}

diesel::table! {
    tracker (id) {
        id -> Int4,
        deploy_inscription_num -> Int8,
        deploy_inscription -> Text,
        ticker -> Text,
        supply -> Numeric,
        supply_minted -> Numeric,
        limit_mint -> Numeric,
        decimals -> Int4,
        holders -> Int8,
        transactions -> Int8,
        inscription_mint_start -> Nullable<Int8>,
        inscription_mint_end -> Nullable<Int8>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(balances, history, inscriptions, status, tracker,);
