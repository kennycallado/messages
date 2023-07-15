use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::tokens::model::{Token, NewToken};
use crate::app::modules::tokens::services::repository as tokens_repository;

pub async fn put_update_admin(db: &Db, _user: UserInClaims, id: i32, new_token: NewToken) -> Result<Json<Token>, Status> {
    let token = tokens_repository::update(db, id, new_token).await;

    match token {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn put_update_user(db: &Db, user: UserInClaims, id: i32, new_token: NewToken) -> Result<Json<Token>, Status> {
    if user.id != new_token.user_id {
        return Err(Status::Unauthorized);
    }

    let token = tokens_repository::update(db, id, new_token).await;

    match token {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Status::InternalServerError),
    }
}
