CREATE TABLE applications (
    application_id ulid NOT NULL DEFAULT gen_ulid() PRIMARY KEY,
    name text NOT NULL,
    allowed_origins text[] NOT NULL,
    allowed_callback_urls text[] NOT NULL,
    owner_id ulid NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users (user_id) ON DELETE CASCADE
);
