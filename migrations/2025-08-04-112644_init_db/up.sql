CREATE TABLE "license_keys"
(
    "id"           SERIAL PRIMARY KEY,
    "key"          TEXT      NOT NULL UNIQUE,
    "expires"      TIMESTAMP NOT NULL,
    "is_activated" BOOL      NOT NULL DEFAULT FALSE,
    "banned"       BOOL,
    "hwid"         TEXT UNIQUE
);

CREATE TABLE "user_info"
(
    "license_id"        INT PRIMARY KEY
        REFERENCES "license_keys" ("id")
            ON DELETE CASCADE,
    "username"          TEXT      NOT NULL UNIQUE,
    "first_login"       TEXT      NOT NULL UNIQUE,
    "last_login"        TEXT      NOT NULL UNIQUE,
    "last_session_time" TIMESTAMP,
    "last_ip"           INET      NOT NULL,
    "os_name"           TEXT,
    "os_version"        TEXT,
    "cpu_info"          TEXT,
    "cpu_cores"         INT,
    "created_at"        TIMESTAMP NOT NULL DEFAULT NOW(),
    "notes"             TEXT
);