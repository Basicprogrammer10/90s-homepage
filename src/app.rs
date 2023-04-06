use parking_lot::{Mutex, MutexGuard};
use rusqlite::Connection;

use crate::db::Database;

pub struct App {
    db: Mutex<Connection>,
}

impl App {
    pub fn new() -> Self {
        let db = Connection::open("./data.db").unwrap();
        db.init();

        Self { db: Mutex::new(db) }
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.db.lock()
    }
}
