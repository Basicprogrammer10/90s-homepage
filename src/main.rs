use std::net::Ipv4Addr;

use afire::{extension::{ServeStatic, Logger}, Middleware, Server, trace::{self, Level}};

fn main() {
    trace::set_log_level(Level::Trace);
    let mut server = Server::<()>::new(Ipv4Addr::LOCALHOST, 8080).keep_alive(false);

    Logger::new().attach(&mut server);
    ServeStatic::new("./web/static").attach(&mut server);

    server.start().unwrap();
}
