use actix_web::{actix::Handler, error, Error};

use crate::database::DbExec;
use super::responses::*;
use super::messages::*;
use super::methods::*;

impl Handler<ListUsers> for DbExec {
    type Result = Result<UserListMsg, Error>;

    fn handle(&mut self, _: ListUsers, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match list(db_conn) {
            Ok(users) => {
                Ok(UserListMsg {
                    users: users,
                })
            },
            Err(e) => {
                Err(error::ErrorInternalServerError(e))
            }
        }
    }
}

impl Handler<GetUserInfo> for DbExec {
    type Result = Result<UserInfoMsg, Error>;

    fn handle(&mut self, user: GetUserInfo, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match find(db_conn, &user.adlogin) {
            Ok(user) => {
                Ok(UserInfoMsg {
                    info: user,
                })
            },
            Err(e) => {
                Err(error::ErrorInternalServerError(e))
            }
        }
    }
}

impl Handler<SetUserInfo> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, info: SetUserInfo, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match update_full(db_conn, &info.user) {
            Ok(_) => {
                Ok(StatusMsg {
                    status: 0,
                    message: String::from("Wow, you've changed!"),
                })
            },
            Err(e) => {
                Err(error::ErrorInternalServerError(e))
            }
        }
    }
}

impl Handler<SetUserPassword> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, value: SetUserPassword, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match update_password(db_conn, &value.adlogin, &value.password) {
            Ok(_) => {
                Ok(StatusMsg {
                    status: 0,
                    message: String::from("New password incoming"),
                })
            },
            Err(e) => {
                Err(error::ErrorInternalServerError(e))
            }
        }
    }
}

impl Handler<GetUserPassword> for DbExec {
    type Result = Result<UserPasswordMsg, Error>;

    fn handle(&mut self, target: GetUserPassword, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match get_password(db_conn, &target.adlogin) {
            Ok(password) => {
                Ok(UserPasswordMsg {
                    password: password,
                })
            },
            Err(e) => {
                Err(error::ErrorInternalServerError(e))
            }
        }
    }
}

/*
impl Handler<AddUser> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, target: GetUserPassword, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;

    }
}
*/