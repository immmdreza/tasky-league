-- Add up migration script here
CREATE TABLE IF NOT EXISTS players (
    id bigserial PRIMARY KEY NOT NULL,
    telegram_id bigint NOT NULL UNIQUE
);