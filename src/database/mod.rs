mod schema;

//pub mod events;
//pub mod groups;
//pub mod shortevents;
//pub mod tokens;
pub mod users;

use diesel::r2d2::{ConnectionManager, Pool};
use actix_web::actix::{Actor, SyncContext};
use diesel::prelude::PgConnection;
use diesel::r2d2::PoolError as Error;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbExec(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExec {
    type Context = SyncContext<Self>;
}

pub fn ignite(db_url: String) -> Result<DbPool, Error> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::builder().build(manager)?;
    Ok(pool)
}
