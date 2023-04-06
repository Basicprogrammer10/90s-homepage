use afire::Request;
use rusqlite::{params, Connection};

pub trait Database {
    fn init(&self);
    fn cleanup(&self);
    fn log_request(&self, req: &Request);
    fn top_pages(&self, count: u32) -> Vec<StatEntry>;
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

    fn top_pages(&self, count: u32) -> Vec<StatEntry> {
        let mut stmt = self
            .prepare(
                "SELECT path, count FROM stats
                WHERE method = 'GET' AND (path LIKE '%/' OR path LIKE '%.html')
                ORDER BY count DESC LIMIT ?",
            )
            .unwrap();

        stmt.query_map([count], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .map(|(url, views)| StatEntry { url, views })
            .collect()
    }
}

pub struct StatEntry {
    pub url: String,
    pub views: u32,
}
