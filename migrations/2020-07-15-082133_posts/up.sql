-- Your SQL goes here

CREATE TABLE post (
    id INTEGER NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 0
)
