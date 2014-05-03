#![crate_id = "evreserve"]

#![feature(phase)]

#[phase(syntax, link)]
extern crate postgres;

extern crate http;

use http::server::Server;

mod server;

fn main() {
    server::EVReserve.serve_forever();
}
