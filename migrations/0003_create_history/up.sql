CREATE TABLE history (
    id SERIAL PRIMARY KEY,
    address_sender TEXT,
    address_receiver TEXT,
    amount NUMERIC NOT NULL,
    ticker TEXT NOT NULL,
    action TEXT NOT NULL,
    invalid BOOLEAN NOT NULL DEFAULT FALSE,
    tx_id TEXT NOT NULL,
    inscription_id TEXT NOT NULL,
    inscription_num BIGINT NOT NULL,
    height BIGINT NOT NULL,
    timestamp BIGINT NOT NULL
)