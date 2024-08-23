-- Add migration script here
CREATE TABLE IF NOT EXISTS channels (
    guild_id BIGINT,
    channel_id BIGINT,
    user_id BIGINT,
    isvocal BOOLEAN DEFAULT TRUE,
    parent BIGINT NULL,
    PRIMARY KEY (guild_id, channel_id),
    FOREIGN KEY (guild_id, parent) REFERENCES channels(guild_id, channel_id)
);