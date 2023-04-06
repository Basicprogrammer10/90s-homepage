use std::sync::Arc;

use afire::{trace, Middleware, Server};
use parking_lot::Mutex;

use crate::app::App;

pub struct Stats {
    app: Arc<App>,
}

impl Stats {
    pub fn new(app: Arc<App>) -> Self {
        Self { app }
    }
}

impl Middleware for Stats {}
