use actix_web::{actix::Handler, error, Error};

use super::messages::*;
use super::methods::*;
use super::responses::*;
use crate::database::DbExec;

impl Handler<ListGroups> for DbExec {
    type Result = Result<GroupListMsg, Error>;

    fn handle(&mut self, _: ListGroups, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match list(db_conn) {
            Ok(groups) => Ok(GroupListMsg { groups: groups }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}
