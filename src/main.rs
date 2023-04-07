use std::{net::Ipv4Addr, process};

use afire::{
    extension::Logger,
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
mod misc;
mod pages;
mod serve_static;
mod stats;

const STATIC_PATH: &str = "./web";

fn main() {
    trace::set_log_level(Level::Trace);
    let mut server = Server::<App>::new(Ipv4Addr::LOCALHOST, 8080)
        .state(App::new())
        .keep_alive(false);
    let app = server.state.as_ref().unwrap().clone();

    Logger::new().attach(&mut server);
    ServeStatic::new(STATIC_PATH).attach(&mut server);
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
