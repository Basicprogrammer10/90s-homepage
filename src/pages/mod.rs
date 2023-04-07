use afire::Server;

use crate::app::App;

mod guest_book;
mod stats;

pub fn attach(server: &mut Server<App>) {
    stats::attach(server);
    guest_book::attach(server);
}
