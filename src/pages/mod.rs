use afire::Server;

use crate::app::App;

mod stats;

pub fn attach(server: &mut Server<App>) {
    stats::attach(server);
}
