pub mod auth;
//pub mod errors;
//pub mod events;
//pub mod groups;
pub mod users;
//pub mod websockets;

use crate::database::{ignite, DbExec};
use crate::middlewares::auth::AuthMiddleware;
use crate::middlewares::authstorage::AuthSessionBackend;
use crate::middlewares::permissions::RoleMiddleware;
use actix::prelude::{Addr, SyncArbiter};
use actix_web::middleware::session::SessionStorage;
use actix_web::{http::Method, middleware::Logger, App};

use auth::*;
use users::*;

pub struct AppState {
    pub db: Addr<DbExec>,
    pub adlogin: Option<String>,
}

pub fn start() -> App<AppState> {
    let db_url = std::env::var("DATABASE_URL").expect(
        "Database connection string must be passed in the DATABASE_URL environment variable",
    );
    let db_pool = ignite(db_url).expect("Database connection failed");

    let db_arbiter = SyncArbiter::start(num_cpus::get(), move || DbExec(db_pool.clone()));

    let state = AppState {
        db: db_arbiter,
        adlogin: None,
    };

    App::with_state(state)
        .middleware(SessionStorage::new(AuthSessionBackend()))
        .middleware(Logger::default())
        .scope("/auth", |r| {
            r.resource("/login", |r| {
                r.post().with(login);
            })
            .resource("/current", |r| {
                r.middleware(AuthMiddleware);
                r.get().a(get_active_user);
            })
            .resource("/password", |r| {
                r.middleware(AuthMiddleware);
                r.post().with(update_password);
            })
        })
        .scope("/users", |r| {
            r.middleware(AuthMiddleware)
                .middleware(RoleMiddleware)
                .resource("", |r| {
                    r.get().with(list_users);
                    r.post().with(add_user)
                })
                .resource("/{adlogin}", |r| {
                    r.get().with(get_user);
                    r.put().with(update_user);
                    r.delete().with(delete_user)
                })
        })
}
