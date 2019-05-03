#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
extern crate jsonwebtoken as jwt;
extern crate actix_web;
extern crate bcrypt;
extern crate dotenv;
extern crate actix;

pub mod app;
pub mod database;

use crate::actix::*;

use actix_web::{actix::System, server};
use crate::app::start;

fn main() {
    let bind_addr = "localhost:40000";

    let sys = System::new("display");

    server::new(app::start)
        .bind(bind_addr).expect(&format!("Failed to bind to the address {}", bind_addr))
        .shutdown_timeout(2)
        .start();

    sys.run();
}
