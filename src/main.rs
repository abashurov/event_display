#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate actix;
extern crate actix_web;
extern crate bcrypt;
extern crate dotenv;
extern crate flexi_logger;
extern crate jsonwebtoken as jwt;

pub mod database;
pub mod middlewares;
pub mod routes;
pub mod utils;

use actix_web::{actix::System, server};
use flexi_logger::{detailed_format, Cleanup, Logger, RotateOver};

fn main() {
    dotenv::dotenv().ok();
    Logger::with_env_or_str("info")
        .format(detailed_format)
        .log_to_file()
        .rotate(RotateOver::Size(5242880), Cleanup::KeepLogFiles(10))
        .start()
        .unwrap();
    let bind_addr = std::env::var("BIND_ADDRESS")
        .expect("Expected address in BIND_ADDRESS variable (e.g. 127.0.0.1:40000)");

    let sys = System::new("display");

    server::new(routes::start)
        .bind(bind_addr.clone())
        .expect(&format!("Failed to bind to the address {}; pass proper address in BIND_ADDRESS variable (e.g. 127.0.0.1:40000)", bind_addr))
        .keep_alive(30)
        .shutdown_timeout(2)
        .start();

    sys.run();
}
