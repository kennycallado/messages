use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::messages::model::Message;
use crate::app::modules::messages::services::repository as messages_repository;

pub async fn get_index_admin(db: &Db, _user: UserInClaims) -> Result<Json<Vec<Message>>, Status> {
    let messages = messages_repository::get_all(&db).await;

    match messages {
        Ok(messages) => Ok(Json(messages)),
        Err(_) => Err(Status::InternalServerError),
    }
}
