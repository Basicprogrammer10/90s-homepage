CREATE TABLE IF NOT EXISTS stats (
    path TEXT NOT NULL,
    count INTEGER NOT NULL,
    UNIQUE (path)
)