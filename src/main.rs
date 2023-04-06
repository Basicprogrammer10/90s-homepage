use std::net::Ipv4Addr;

use afire::{extension::{ServeStatic, Logger}, Middleware, Server, trace::{self, Level}};
use app::App;
use stats::Stats;

mod app;
mod stats;

fn main() {
    trace::set_log_level(Level::Trace);
    let app = App::new();
    let mut server = Server::<App>::new(Ipv4Addr::LOCALHOST, 8080).state(app).keep_alive(false);

    Logger::new().attach(&mut server);
    Stats::new(server.state.as_ref().unwrap().clone()).attach(&mut server);
    ServeStatic::new("./web/static").attach(&mut server);

    server.start().unwrap();
}
