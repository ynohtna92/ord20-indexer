CREATE TABLE inscriptions (
    id SERIAL PRIMARY KEY,
    genesis_tx_id TEXT NOT NULL,
    genesis_address TEXT NOT NULL,
    address_sender TEXT,
    address_receiver TEXT,
    ticker TEXT NOT NULL,
    action TEXT NOT NULL,
    supply NUMERIC,
    limit_mint NUMERIC,
    decimal INT,
    amount NUMERIC,
    inscription_id TEXT UNIQUE NOT NULL,
    inscription_num BIGINT UNIQUE NOT NULL,
    height BIGINT NOT NULL,
    timestamp BIGINT NOT NULL,
    output TEXT UNIQUE NOT NULL,
    value BIGINT,
    valid BOOLEAN DEFAULT FALSE,
    spent BOOLEAN DEFAULT FALSE,
    spent_tx TEXT,
    spent_offset BIGINT,
    spent_height BIGINT,
    spent_timestamp BIGINT
);

CREATE INDEX idx_inscriptions_genesis_address ON inscriptions(genesis_address);
CREATE INDEX idx_inscriptions_spent ON inscriptions(spent);