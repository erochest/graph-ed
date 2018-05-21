CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    nickname VARCHAR(25) DEFAULT NULL,
    email VARCHAR(128) NOT NULL UNIQUE,
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    last_login TIMESTAMP DEFAULT NULL
);