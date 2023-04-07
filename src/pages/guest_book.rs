use std::borrow::Borrow;

use afire::{HeaderType, Method, Query, Response, Server, Status};

use askama::Template;

use crate::{
    app::App,
    db::{Database, GuestBookEntry},
};

#[derive(Template)]
#[template(path = "guestbook.html")]
struct GuestBookTemplate {
    entries: Vec<GuestBookEntry>,
}

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/guestbook", |app, req| {
        let data = Query::from_body(String::from_utf8_lossy(&req.body).borrow());
        let name = data.get("name").expect("No name supplied");
        let message = data.get("message").expect("No message supplied");
        app.db().add_guestbook(name, message);

        Response::new()
            .status(Status::Found)
            .header(HeaderType::Location, "/guestbook")
    });

    server.stateful_route(Method::GET, "/guestbook", |app, _req| {
        let template = GuestBookTemplate {
            entries: app.db().get_guestbook_entries(),
        };
        Response::new().text(template.render().unwrap())
    });
}
