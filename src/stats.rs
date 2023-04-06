use std::sync::Arc;

use afire::{Middleware, Request, Response};

use crate::{app::App, db::Database};

pub struct Stats {
    pub app: Arc<App>,
}

impl Stats {
    pub fn new(app: Arc<App>) -> Self {
        Self { app }
    }
}

impl Middleware for Stats {
    fn end(&self, req: &Request, _res: &Response) {
        self.app.db().log_request(&req);
    }
}
