#[crate_id = "evreserve"];

extern crate http;

use http::server::Server;

mod server;

fn main() {
    server::EVReserve.serve_forever();
}
