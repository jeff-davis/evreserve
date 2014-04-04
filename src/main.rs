#![crate_id = "evreserve"]
#![feature(macro_rules)]

extern crate postgres;
extern crate http;

use http::server::Server;

mod server;

fn main() {
    server::EVReserve.serve_forever();
}
