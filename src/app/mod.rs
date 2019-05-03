pub mod auth;
//pub mod errors;
//pub mod events;
//pub mod groups;
//pub mod users;
//pub mod websockets;

use actix_web::{{middleware::Logger}, App, HttpRequest, http::{header, Method},};
use crate::database::{ignite, DbExec};
use actix::prelude::{Addr, SyncArbiter};

use auth::login;

pub struct AppState {
    db: Addr<DbExec>,
    secret: String,
}

pub fn start() -> App<AppState> {
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT secret must be passed in the JWT_SECRET environment variable");
    let db_url = std::env::var("DATABASE_URL")
        .expect("Database connection string must be passed in the DATABASE_URL environment variable");
    let db_pool = ignite(db_url)
        .expect("Database connection failed");

    let db_arbiter = SyncArbiter::start(num_cpus::get(), move || DbExec(db_pool.clone()));

    let state = AppState {
        db: db_arbiter,
        secret: jwt_secret,
    };

    App::with_state(state)
        .middleware(Logger::default())
        .resource("/login", | r | {
            r.method(Method::POST).with(login);
        })
}