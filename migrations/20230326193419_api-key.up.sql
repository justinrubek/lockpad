-- Add up migration script here
CREATE TABLE api_keys (
    api_key_id ulid NOT NULL DEFAULT gen_ulid() PRIMARY KEY,
    owner_id ulid NOT NULL,
    name text NOT NULL,
    secret text NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users (user_id) ON DELETE CASCADE
);
