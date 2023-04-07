use std::time::{SystemTime, UNIX_EPOCH};

use afire::Request;
use rusqlite::Connection;

use crate::misc;

pub trait Database {
    // == Base ==
    fn init(&self);
    fn cleanup(&self);

    // == Stats ==
    fn log_request(&self, req: &Request);
    fn top_pages(&self, count: u32) -> Vec<StatEntry>;

    // == Guest Book ==
    fn add_guestbook(&self, name: &str, message: &str);
    fn get_guestbook_entries(&self) -> Vec<GuestBookEntry>;
}

impl Database for Connection {
    fn init(&self) {
        self.execute(
            "CREATE TABLE IF NOT EXISTS stats (
                path TEXT NOT NULL,
                count INTEGER NOT NULL,
                UNIQUE (path)
            )",
            [],
        )
        .unwrap();
        self.execute(
            "CREATE TABLE IF NOT EXISTS guestbook (
                name TEXT NOT NULL,
                message TEXT NOT NULL,
                date INTEGER NOT NULL
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
            "INSERT INTO stats (path, count) VALUES (?, 1)
            ON CONFLICT (path) DO UPDATE SET count = count + 1",
            [&req.path],
        )
        .unwrap();
    }

    fn top_pages(&self, count: u32) -> Vec<StatEntry> {
        let mut stmt = self
            .prepare(
                "SELECT path, count FROM stats
                ORDER BY count DESC LIMIT ?",
            )
            .unwrap();

        stmt.query_map([count], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(Result::unwrap)
            .map(|(url, views)| StatEntry { url, views })
            .collect()
    }

    fn add_guestbook(&self, name: &str, message: &str) {
        self.execute(
            "INSERT INTO guestbook VALUES (?, ?, strftime('%s','now'))",
            [name, message],
        )
        .unwrap();
    }

    fn get_guestbook_entries(&self) -> Vec<GuestBookEntry> {
        let mut stmt = self
            .prepare("SELECT name, message, date FROM guestbook ORDER BY date DESC")
            .unwrap();

        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .unwrap()
            .map(Result::unwrap)
            .map(|(name, message, date)| GuestBookEntry {
                name,
                message,
                date: misc::best_time(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                        .saturating_sub(date),
                ),
            })
            .collect()
    }
}

pub struct StatEntry {
    pub url: String,
    pub views: u32,
}

pub struct GuestBookEntry {
    pub name: String,
    pub message: String,
    pub date: String,
}
