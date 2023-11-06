use rocket::http::Status;
use rocket::serde::json::Json;

use crate::app::providers::guards::claims::AccessClaims;
use crate::database::connection::Db;

use crate::app::modules::messages::model::{Message, NewMessage};
use crate::app::modules::messages::handlers::{index, show, create, update};


pub fn routes() -> Vec<rocket::Route> {
    routes![
        options_all,
        get_index,
        get_index_none,
        get_show,
        get_show_none,
        post_create,
        post_create_none,
        put_update,
        put_update_none,
    ]
}

#[options("/<_..>")]
pub fn options_all() -> Status {
    Status::Ok
}

#[get("/", rank = 1)]
pub async fn get_index(db: &Db, claims: AccessClaims) -> Result<Json<Vec<Message>>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => index::get_index_admin(db, claims.0.user).await,
        _ => {
            println!("Error: get_index; Role not handled {}", claims.0.user.role.name);
            Err(Status::Unauthorized)
        },
    }
}

#[get("/", rank = 2)]
pub fn get_index_none() -> Status {
    Status::Unauthorized
}

#[get("/<id>", rank = 101)]
pub async fn get_show(db: &Db, claims: AccessClaims, id: i32) -> Result<Json<Message>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => show::get_show_admin(db, claims.0.user, id).await,
        _ => {
            println!("Error: get_show; Role not handled {}", claims.0.user.role.name);
            Err(Status::Unauthorized)
        },
    }
}

#[get("/<_id>", rank = 102)]
pub fn get_show_none(_id: i32) -> Status {
    Status::Unauthorized
}

#[post("/", data = "<message>", rank = 1)]
pub async fn post_create(db: &Db, claims: AccessClaims, message: Json<NewMessage>) -> Result<Json<Message>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => create::post_create_admin(db, claims.0.user, message.into_inner()).await,
        _ => {
            println!("Error: post_create; Role not handled {}", claims.0.user.role.name);
            Err(Status::Unauthorized)
        },
    }
}

#[post("/", data = "<_message>", rank = 2)]
pub fn post_create_none(_message: Json<NewMessage>) -> Status {
    Status::Unauthorized
}

#[put("/<id>", data = "<message>", rank = 101)]
pub async fn put_update(db: &Db, claims: AccessClaims, id: i32, message: Json<NewMessage>) -> Result<Json<Message>, Status> {
    match claims.0.user.role.name.as_str() {
        "admin" => update::put_update_admin(db, claims.0.user, id, message.into_inner()).await,
        _ => {
            println!("Error: put_update; Role not handled {}", claims.0.user.role.name);
            Err(Status::Unauthorized)
        },
    }
}

#[put("/<_id>", data = "<_message>", rank = 102)]
pub fn put_update_none(_id: i32, _message: Json<Message>) -> Status {
    Status::Unauthorized
}
