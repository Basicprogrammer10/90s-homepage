use std::process;

use afire::{
    trace::{self, Level},
    Middleware, Server,
};
use app::App;
use owo_colors::OwoColorize;
use serve_static::ServeStatic;
use stats::Stats;

use crate::db::Database;

mod app;
mod db;
mod logger;
mod misc;
mod pages;
mod serve_static;
mod stats;
use logger::Logger;

fn main() {
    trace::set_log_level(Level::Trace);
    let app = App::new();
    let mut server = Server::<App>::new(app.config.host.as_str(), app.config.port)
        .state(app)
        .keep_alive(false);
    let app = server.state.as_ref().unwrap().clone();

    Logger.attach(&mut server);
    ServeStatic::new(&app.config.static_path).attach(&mut server);
    Stats::new(app.clone()).attach(&mut server);

    pages::attach(&mut server);

    ctrlc::set_handler(move || {
        println!("{}", "[*] Exiting".yellow());
        app.db().cleanup();
        process::exit(0);
    })
    .unwrap();

    server.start().unwrap();
}
