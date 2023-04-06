use parking_lot::Mutex;
use rusqlite::Connection;

pub struct App {
    db: Mutex<Connection>,
}

impl App {
    pub fn new() -> Self {
        let db = Connection::open("./data.db").unwrap();

        Self { db: Mutex::new(db) }
    }
}
