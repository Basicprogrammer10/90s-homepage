use std::process;

use afire::{
    extension::Date,
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
    let threads = app.config.threads;

    let mut server = Server::<App>::new(app.config.host.as_str(), app.config.port).state(app);
    let app = server.app();

    Date.attach(&mut server);
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

    server.start_threaded(threads).unwrap();
}
