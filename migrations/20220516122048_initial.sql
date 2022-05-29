-- Add migration script here
CREATE TABLE IF NOT EXISTS forecasts
(
    id          INTEGER PRIMARY KEY NOT NULL,
    name        TEXT                NOT NULL
);