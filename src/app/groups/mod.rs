use actix_web::{error, http, App, HttpRequest, HttpResponse};

use database::methods::groups;
use database::models::EventGroup;

struct ListGroups {}

impl Message for ListGroups {
    type Result = Result<Vec<EventGroup>, Error>;
}

impl Handler<ListGroups> for DbExecutor {
    type Result = Result<Vec<EventGroup>, Error>;

    fn handle(&mut self, msg: ListGroups, _: &mut Self::Context) -> Self::Result {
        groups::list()
    }
}
