CREATE TABLE status (
    id SERIAL PRIMARY KEY,
    key TEXT UNIQUE NOT NULL,
    value TEXT
);

INSERT INTO status (key, value) VALUES ('last_inscription','224060');
INSERT INTO status (key, value) VALUES ('last_height','2465225');