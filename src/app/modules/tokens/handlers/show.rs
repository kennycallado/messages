use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::tokens::model::Token;
use crate::app::modules::tokens::services::repository as tokens_repository;

pub async fn get_show_admin(
    db: &Db,
    _user: UserInClaims,
    id: i32,
) -> Result<Json<Token>, Status> {
    let token = tokens_repository::get_by_id(db, id).await;

    match token {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Status::InternalServerError),
    }
}
