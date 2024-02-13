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
);

CREATE INDEX idx_history_address_sender ON history(address_sender);
CREATE INDEX idx_history_address_receiver ON history(address_receiver);
CREATE INDEX idx_history_ticker ON history(ticker);