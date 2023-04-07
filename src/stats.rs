use std::sync::Arc;

use afire::{prelude::MiddleResult, Middleware, Request, Response, Status};

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
    fn post(&self, req: &Request, res: &mut Response) -> MiddleResult {
        if req.path.contains('.') || res.status != Status::Ok {
            return MiddleResult::Continue;
        }

        self.app.db().log_request(req);
        MiddleResult::Continue
    }
}
