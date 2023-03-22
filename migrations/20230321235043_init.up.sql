CREATE EXTENSION ulid;

CREATE TABLE IF NOT EXISTS users (
    user_id ulid NOT NULL DEFAULT gen_ulid() PRIMARY KEY,
    identifier text NOT NULL UNIQUE,
    secret text NOT NULL
);
