CREATE EXTENSION ulid;

CREATE TABLE IF NOT EXISTS users (
    user_id ulid NOT NULL DEFAULT gen_ulid() PRIMARY KEY,
    name text NOT NULL,
    secret text NOT NULL
);
