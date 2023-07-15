use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::tokens::model::Token;
use crate::app::modules::tokens::services::repository as tokens_repository;

pub async fn get_index_admin(db: &Db, _user: UserInClaims) -> Result<Json<Vec<Token>>, Status> {
    let tokens = tokens_repository::get_all(db).await;

    match tokens {
        Ok(tokens) => Ok(Json(tokens)),
        Err(_) => Err(Status::InternalServerError),
    }
}
