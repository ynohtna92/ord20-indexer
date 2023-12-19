CREATE TABLE balances (
    id SERIAL PRIMARY KEY,
    address TEXT NOT NULL,
    ticker TEXT NOT NULL,
    balance NUMERIC NOT NULL DEFAULT 0,
    transfer_balance NUMERIC NOT NULL DEFAULT 0,
    CONSTRAINT unique_address_ticker_pair UNIQUE (address, ticker)
)