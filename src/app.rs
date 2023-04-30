use std::{env, fs, path::PathBuf};

use parking_lot::{Mutex, MutexGuard};
use rusqlite::Connection;
use serde::Deserialize;

use crate::db::Database;

pub struct App {
    db: Mutex<Connection>,
    pub config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub threads: usize,

    pub db_path: String,
    pub static_path: String,
}

impl App {
    pub fn new() -> Self {
        let config_file = PathBuf::from(
            env::args()
                .nth(1)
                .unwrap_or_else(|| "./config.toml".to_string()),
        );
        let config = toml::from_str::<Config>(&fs::read_to_string(config_file).unwrap()).unwrap();

        let mut db = Connection::open(&config.db_path).unwrap();
        db.init();

        Self {
            db: Mutex::new(db),
            config,
        }
    }

    pub fn db(&self) -> MutexGuard<Connection> {
        self.db.lock()
    }
}
