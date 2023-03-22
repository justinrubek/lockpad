CREATE EXTENSION ulid;

CREATE TABLE IF NOT EXISTS users (
    id serial PRIMARY KEY,
    name text NOT NULL,
    secret text NOT NULL
);
