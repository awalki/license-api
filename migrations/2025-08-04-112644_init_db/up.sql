CREATE TABLE "license_keys"
(
    "id"           SERIAL PRIMARY KEY,
    "key"          TEXT NOT NULL UNIQUE,
    "expires"  TIMESTAMP NOT NULL,
    "is_activated" BOOL NOT NULL DEFAULT FALSE,
    "hwid"         TEXT UNIQUE,
    "description"  TEXT
);