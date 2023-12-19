CREATE TABLE tracker (
    id SERIAL PRIMARY KEY,
    deploy_inscription_num BIGINT NOT NULL,
    deploy_inscription TEXT NOT NULL,
    ticker TEXT UNIQUE NOT NULL,
    supply NUMERIC NOT NULL,
    supply_minted NUMERIC NOT NULL DEFAULT 0,
    limit_mint NUMERIC NOT NULL,
    decimals INT NOT NULL DEFAULT 18,
    holders BIGINT NOT NULL DEFAULT 0,
    transactions BIGINT NOT NULL DEFAULT 0,
    inscription_mint_start BIGINT,
    inscription_mint_end BIGINT
)