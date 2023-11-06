use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::messages::model::NewMessage;
use crate::app::modules::tokens::model::Token;

use crate::app::modules::dispatcher::services::config_getter::ConfigGetter;
use crate::app::modules::dispatcher::handlers::sender;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_all,
        get_send_id,
        get_send_none,
        post_send_message,
        post_send_none,
    ]
}

#[options("/<_..>")]
pub fn options_all() -> Status {
    Status::Ok
}

#[get("/<message_id>/user/<user_id>", rank = 101)]
pub async fn get_send_id(db: &Db, claims: AccessClaims, user_id: i32, message_id: i32)
    -> Result<Status, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => sender::send_message_id(db, claims.0.user, user_id, message_id).await,
        _ => {
            println!("Error: send_message_id; Role not handled {}", claims.0.user.role.name);
            Err(Status::Unauthorized)
        }
    }
} 

#[get("/<_message_id>/user/<_user_id>", rank = 102)]
pub fn get_send_none(_message_id: i32, _user_id: i32) -> Status {
    Status::Unauthorized
}


#[post("/user/<user_id>", data = "<message>")]
pub async fn post_send_message(db: &Db, claims: AccessClaims, user_id: i32, message: Json<NewMessage>)
    -> Result<Status, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" |
        "robot" => sender::send_message(db, claims.0.user, user_id, message.into_inner()).await,
        _ => {
            println!("Error: send_message; Role not handled {}", claims.0.user.role.name);
            Err(Status::Unauthorized)
        }
    }
}

#[post("/user/<_user_id>", rank = 102)]
pub fn post_send_none(_user_id: i32) -> Status {
    Status::Unauthorized
}

/* endpoint lista de mensajes enviados a un usuario */
