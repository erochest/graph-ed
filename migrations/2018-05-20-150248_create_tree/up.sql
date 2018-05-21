CREATE TABLE trees (
    id SERIAL PRIMARY KEY,
    title VARCHAR(128),
    node_id INTEGER NOT NULL REFERENCES nodes (id),
    created TIMESTAMP NOT NULL DEFAULT NOW(),
    updated TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE user_trees (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users (id),
    tree_id INTEGER NOT NULL REFERENCES trees (id)
);
