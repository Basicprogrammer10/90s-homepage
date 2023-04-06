use afire::{Method, Response, Server};

use askama::Template;

use crate::{
    app::App,
    db::{Database, StatEntry},
};

const TOP_PAGES: u32 = 10;

#[derive(Template)]
#[template(path = "../web/templates/stats.html")]
struct StatsTemplate {
    pages: Vec<StatEntry>,
}

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/stats", |app, _res| {
        let stats = app.db().top_pages(TOP_PAGES);
        let template = StatsTemplate { pages: stats };
        Response::new().text(template.render().unwrap())
    });
}
