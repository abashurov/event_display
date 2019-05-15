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
use flexi_logger::{detailed_format, Logger};

fn main() {
    dotenv::dotenv().ok();
    Logger::with_env_or_str("debug")
        .format(detailed_format)
        .start()
        .unwrap();
    let bind_addr = "localhost:40000";

    let sys = System::new("display");

    server::new(routes::start)
        .bind(bind_addr)
        .expect(&format!("Failed to bind to the address {}", bind_addr))
        .keep_alive(30)
        .shutdown_timeout(2)
        .start();

    sys.run();
}
