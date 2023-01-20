-- Add up migration script here
CREATE TABLE IF NOT EXISTS players_info (
    id bigserial PRIMARY KEY NOT NULL,
    telegram_id bigint NOT NULL UNIQUE,
    kindness real NOT NULL,
    skill real NOT NULL,

    player_id bigserial REFERENCES players(id)
);