use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::messages::model::{Message, NewMessage};
use crate::app::modules::messages::services::repository as messages_repository;

pub async fn put_update_admin(db: &Db, _user: UserInClaims, id: i32, new_message: NewMessage) -> Result<Json<Message>, Status> {
    let message = messages_repository::update(&db, id, new_message).await;

    match message {
        Ok(message) => Ok(Json(message)),
        Err(_) => Err(Status::InternalServerError),
    }
}
