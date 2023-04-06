use afire::Request;
use rusqlite::{params, Connection};

pub trait Database {
    fn init(&self);
    fn cleanup(&self);
    fn log_request(&self, req: &Request);
}

impl Database for Connection {
    fn init(&self) {
        self.execute(
            "CREATE TABLE IF NOT EXISTS stats (
                method TEXT NOT NULL,
                path TEXT NOT NULL,
                count INTEGER NOT NULL,
                UNIQUE (method, path)
            )",
            [],
        )
        .unwrap();
        self.pragma_update(None, "journal_mode", "WAL").unwrap();
        self.pragma_update(None, "synchronous", "NORMAL").unwrap();
    }

    fn cleanup(&self) {
        self.pragma_update(None, "wal_checkpoint", "TRUNCATE")
            .unwrap();
    }

    fn log_request(&self, req: &Request) {
        self.execute(
            "INSERT INTO stats (method, path, count) VALUES (?, ?, 1)
            ON CONFLICT (method, path) DO UPDATE SET count = count + 1",
            params![req.method.to_string(), req.path],
        )
        .unwrap();
    }
}
