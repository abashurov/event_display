pub mod groups;
//pub mod errors;
pub mod auth;
pub mod events;
pub mod users;
//pub mod websockets;


use actix_web::{http::Method, middleware::Logger, App};
use actix_web::middleware::session::SessionStorage;
use actix::prelude::{Addr, SyncArbiter};

use crate::middlewares::authstorage::AuthSessionBackend;
//use crate::middlewares::permissions::RoleMiddleware;
use crate::middlewares::auth::AuthMiddleware;
use crate::middlewares::role::RoleMiddleware;
use crate::database::{ignite, DbExec};


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
                r.post().with(auth::login);
            })
            .resource("/current", |r| {
                r.middleware(AuthMiddleware);
                r.get().a(auth::get_active_user);
            })
            .resource("/password", |r| {
                r.middleware(AuthMiddleware);
                r.post().with(auth::update_password);
            })
        })
        .scope("/users", |r| {
            r.middleware(AuthMiddleware)
                .middleware(RoleMiddleware::with_display_or_above())
                .resource("", |r| {
                    r.get().with(users::list_users)
                })
                .resource("/{adlogin}", |r| {
                    r.get().with(users::get_user)
                })
                /* actix-web does not have per-route middlewares due to the impl on the Middleware */
                .nested("/protected", |r| {
                    r.middleware(RoleMiddleware::with_manager_or_above())
                        .resource("", |r| {
                            r.get().with(users::list_users);
                            r.post().with(users::add_user)
                        })
                        .resource("/{adlogin}", |r| {
                            r.get().with(users::get_user);
                            r.put().with(users::update_user);
                            r.delete().with(users::delete_user)
                        })
                })
        })
        .scope("/groups", |r| {
            r.middleware(AuthMiddleware)
                .resource("", |r| r.get().with(groups::list_groups))
        })
        .scope("/events", |r| {
            r.resource("/my", |r| {
                r.middleware(AuthMiddleware);
                r.get().with(events::list_assignee_events)
            })
            .middleware(AuthMiddleware)
            .resource("/groups/{group_id}", |r| {
                r.middleware(RoleMiddleware::with_display_or_above());
                r.get().with(events::list_events)
            })
            .resource("/{event_id}", |r| {
                r.middleware(RoleMiddleware::with_display_or_above());
                r.get().with(events::get_event)
            })
            .resource("/{event_id}/assignees", |r| {
                r.middleware(RoleMiddleware::with_display_or_above());
                r.get().with(events::list_event_assignees)
            })
            .nested("/protected", |r| {
                r.middleware(RoleMiddleware::with_manager_or_above())
                    .resource("/groups/{group_id}", |r| {
                        r.get().with(events::list_events);
                        r.post().with(events::add_event)
                    })
                    .resource("/{event_id}", |r| {
                        r.get().with(events::get_event);
                        r.delete().with(events::delete_event)
                    })
                    .resource("/{event_id}/assignees", |r| {
                        r.get().with(events::list_event_assignees)
                    })
                    .resource("/{event_id}/assignees/{user_name}", |r| {
                        r.post().with(events::add_event_assignee);
                        r.delete().with(events::delete_event_assignee)
                    })
            })
        })
}
