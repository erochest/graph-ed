CREATE TABLE nodes (
    id SERIAL PRIMARY KEY,
    title VARCHAR(128),
    content TEXT,
    node_id INTEGER DEFAULT NULL REFERENCES nodes (id),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    updated TIMESTAMP NOT NULL DEFAULT NOW()
);
