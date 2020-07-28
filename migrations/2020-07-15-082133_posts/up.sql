CREATE TABLE "image" (
    id INTEGER NOT NULL PRIMARY KEY,
    file_name TEXT,
    web_name TEXT NOT NULL,
    path TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT (datetime(CURRENT_TIMESTAMP, 'localtime'))
);

CREATE UNIQUE INDEX IF NOT EXISTS web_name_idx on image(web_name);

CREATE TABLE post (
    id INTEGER NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    intro TEXT NOT NULL,
    web_name TEXT NOT NULL,
    banner INTEGER,
    body TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT (datetime(CURRENT_TIMESTAMP, 'localtime')),
    published_at DATETIME,
    published BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (banner) REFERENCES image(id)
);

CREATE UNIQUE INDEX IF NOT EXISTS web_name_idx on post(web_name);
CREATE INDEX IF NOT EXISTS  published_idx on post(published_at);
