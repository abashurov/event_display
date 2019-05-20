use actix_web::{actix::Handler, error, Error};

use super::methods::*;
use super::messages::*;
use super::models::User;
use super::responses::*;

use crate::database::DbExec;

impl Handler<ListUsers> for DbExec {
    type Result = Result<UserListMsg, Error>;

    fn handle(&mut self, _: ListUsers, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match list(db_conn) {
            Ok(users) => Ok(UserListMsg { result: users }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<GetUserInfo> for DbExec {
    type Result = Result<UserInfoMsg, Error>;

    fn handle(&mut self, user: GetUserInfo, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match find(db_conn, &user.adlogin) {
            Ok(user) => Ok(UserInfoMsg { result: user }),
            Err(e) => match e {
                diesel::result::Error::NotFound => Err(error::ErrorNotFound("No such user exists")),
                _ => {
                    warn!("User could not be listed: {}", e);
                    Err(error::ErrorInternalServerError("Could not find user"))
                }
            },
        }
    }
}

impl Handler<SetUserInfo> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, info: SetUserInfo, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match update_partial(db_conn, &info.user) {
            Ok(_) => Ok(StatusMsg {
                status: 0,
                message: String::from("Wow, you've changed!"),
            }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<SetUserPassword> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, value: SetUserPassword, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match bcrypt::hash(value.password, bcrypt::DEFAULT_COST) {
            Ok(hashed_password) => {
                match update_password(db_conn, &value.adlogin, &hashed_password) {
                    Ok(_) => Ok(StatusMsg {
                        status: 0,
                        message: String::from("New password incoming"),
                    }),
                    Err(e) => Err(error::ErrorInternalServerError(e)),
                }
            }
            Err(e) => {
                warn!("Failed to generate hashed password: {}", e);
                Err(error::ErrorInternalServerError("Failed to save password"))
            }
        }
    }
}

impl Handler<GetUserPassword> for DbExec {
    type Result = Result<UserPasswordMsg, Error>;

    fn handle(&mut self, target: GetUserPassword, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match get_password(db_conn, &target.adlogin) {
            Ok(password) => Ok(UserPasswordMsg { password: password }),
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<AddUser> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, target: AddUser, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match bcrypt::hash(target.new_user.password, bcrypt::DEFAULT_COST) {
            Ok(hashed_password) => {
                let new_user = User {
                    adlogin: target.new_user.adlogin,
                    display_name: target.new_user.display_name,
                    absent: target.new_user.absent,
                    password: hashed_password,
                    role: target.new_user.role,
                    availability: target.new_user.availability,
                };
                match add(db_conn, &new_user) {
                    Ok(_) => Ok(StatusMsg {
                        status: 0,
                        message: String::from("New user created"),
                    }),
                    Err(e) => {
                        warn!("New user record could not be inserted: {}", e);
                        Err(error::ErrorInternalServerError("Could not insert record"))
                    }
                }
            }
            Err(e) => Err(error::ErrorInternalServerError(e)),
        }
    }
}

impl Handler<DeleteUser> for DbExec {
    type Result = Result<StatusMsg, Error>;

    fn handle(&mut self, target: DeleteUser, _: &mut Self::Context) -> Self::Result {
        let db_conn = &self.0.get().map_err(error::ErrorInternalServerError)?;
        match delete(db_conn, &target.adlogin) {
            Ok(_) => Ok(StatusMsg {
                status: 0,
                message: String::from("User is no more"),
            }),
            Err(e) => match e {
                diesel::result::Error::NotFound => Err(error::ErrorNotFound("No such user exists")),
                _ => {
                    warn!("User could not be deleted: {}", e);
                    Err(error::ErrorInternalServerError("Could not delete user"))
                }
            },
        }
    }
}
