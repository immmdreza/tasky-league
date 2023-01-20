-- Add up migration script here
CREATE TABLE IF NOT EXISTS player_preferences (
    id bigserial PRIMARY KEY NOT NULL,
    minimum_player_count int,
    maximum_player_count int,
    with_jurors boolean,
    force_preferences boolean NOT NULL,

    player_id bigserial REFERENCES players(id)
);