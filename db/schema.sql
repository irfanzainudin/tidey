CREATE TABLE IF NOT EXISTS tideys (
    id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    body        TEXT NOT NULL,
    created     NUMERIC NOT NULL,
    updated     NUMERIC NOT NULL
);
