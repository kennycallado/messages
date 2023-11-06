use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::tokens::model::{NewToken, Token};
use crate::app::modules::tokens::services::repository as tokens_repository;

pub async fn post_create_admin(
    db: &Db,
    _user: UserInClaims,
    new_token: NewToken,
) -> Result<Json<Token>, Status> {
    let token = tokens_repository::create(db, new_token).await;

    match token {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Status::InternalServerError),
    }
}
