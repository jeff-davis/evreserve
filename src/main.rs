#![crate_id = "evreserve"]

#![feature(phase)]

extern crate http;

#[phase(plugin, link)]
extern crate postgres;

use http::server::Server;

mod server;
mod app;

fn main() {
    server::EVReserve.serve_forever();
}
