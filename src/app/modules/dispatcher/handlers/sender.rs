use rocket::http::Status;

use base64ct::{Base64UrlUnpadded, Encoding};
use web_push_native::{
    jwt_simple::algorithms::ES256KeyPair, p256::PublicKey, Auth, Error, WebPushBuilder,
};

use hyper::{header, Body, Client, StatusCode};
use hyper_rustls::HttpsConnectorBuilder;

use crate::app::providers::guards::claims::AccessClaims;
use crate::app::providers::services::claims::UserInClaims;
use crate::database::connection::Db;

use crate::app::modules::dispatcher::services::config_getter::ConfigGetter;
use crate::app::modules::messages::model::NewMessage;
use crate::app::modules::tokens::model::WebPushToken;

use crate::app::modules::messages::services::repository as message_repository;
use crate::app::modules::tokens::services::repository as token_repository;

pub async fn send_message_id(
    db: &Db,
    _user: UserInClaims,
    user_id: i32,
    message_id: i32,
) -> Result<Status, Status> {
    // get the message
    let message = match message_repository::get_by_id(db, message_id).await {
        Ok(message) => message,
        Err(_) => {
            println!("Error: sender; There is no message with id {}", message_id);
            return Err(Status::InternalServerError);
        }
    };

    // get the token
    let token = match token_repository::get_by_user_id(db, user_id).await {
        Ok(token) => token,
        Err(_) => {
            println!("Error: sender; There is no token for user {}", user_id);
            return Err(Status::InternalServerError);
        }
    };

    match (&token.fcm_token, &token.web_token) {
        (None, Some(token)) => to_web_push(token.into(), message.into()).await,
        (Some(token), None) => to_fcm(token, message.into()).await,
        (Some(_), Some(_)) => {
            // TODO: send to both ??
            println!(
                "Error: sender; There is both fcm and web token for user {}",
                user_id
            );
            return Err(Status::InternalServerError);
        }
        (None, None) => {
            println!("Error: sender; There is no token for user {}", user_id);
            return Err(Status::InternalServerError);
        }
    }
}

pub async fn send_message(
    db: &Db,
    _user: UserInClaims,
    user_id: i32,
    message: NewMessage,
) -> Result<Status, Status> {
    // get the token
    // determine if fcm or web push
    // send the message
    // event source
    // return status
    unimplemented!()
}

async fn to_web_push(token: WebPushToken, message: NewMessage) -> Result<Status, Status> {
    let api_key = match ConfigGetter::get_vapid_key() {
        Some(api_key) => api_key,
        None => {
            println!("Error: sender; There is no api key for WEB PUSH");
            return Err(Status::InternalServerError);
        }
    };

    // TODO: take care of the unwraps
    // let message: rocket::serde::json::Value = message.into();
    // println!("message: {}", message);

    let key_pair =
        ES256KeyPair::from_bytes(&Base64UrlUnpadded::decode_vec(api_key.as_str()).unwrap())
            .unwrap();
    // let builder = WebPushBuilder::new(
    //     token.endpoint.parse().unwrap(),
    //     PublicKey::from_sec1_bytes(&Base64UrlUnpadded::decode_vec(&token.p256dh).unwrap()).unwrap(),
    //     Auth::clone_from_slice(&Base64UrlUnpadded::decode_vec(&token.auth).unwrap()),
    // ).with_vapid(&key_pair, "mailto:kennycallado@hotmail.com");
    //
    let builder = WebPushBuilder::new(
        token.endpoint.parse().unwrap(),
        PublicKey::from_sec1_bytes(&Base64UrlUnpadded::decode_vec(&token.p256dh).unwrap())
            .unwrap(),
        Auth::clone_from_slice(&Base64UrlUnpadded::decode_vec(&token.auth).unwrap()),
    )
    .with_vapid(&key_pair, "mailto:kennycallado@hotmail.com");

    // Esto no estaba funcionando
    // let message = rocket::serde::json::Value::from(r#"{ "notification": { "title": "Hello", "body": "World", "icon": "assets/icons/icon-72x72.png", "vibrate": [100, 50, 100], "data": { "primaryKey": 1, "type": "info", "content": ["content"] }, "actions": [{ "action": "explore", "title": "Explore this new world" }] } }"#);
    // println!("message: {:#?}", message);

    let message: rocket::serde::json::Value = message.into();
    let web_push = builder
        .build(message.to_string())
        .unwrap()
        .map(|body| body.into());

    // let blah = r#"{ "notification": { "title": "Hello", "body": "World", "icon": "assets/icons/icon-72x72.png", "vibrate": [100, 50, 100], "data": { "primaryKey": 1, "type": "info", "content": ["content"] }, "actions": [{ "action": "explore", "title": "Explore this new world" }] } }"#;
    // let web_push = builder.build(blah).unwrap().map(|body| body.into());

    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build();
    let client: Client<_, Body> = Client::builder().build(https);

    let res = match client.request(web_push).await {
        Ok(response) => Ok(response),
        Err(e) => {
            println!("Error: {}", e);
            Err(Status::InternalServerError)
        }
    };

    match res {
        Ok(res) => {
            if !res.status().is_success() {
                return Err(Status::from_code(res.status().as_u16()).unwrap());
            }

            Ok(Status::Ok)
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

async fn to_fcm(token: &str, message: NewMessage) -> Result<Status, Status> {
    let api_key = match ConfigGetter::get_fcm_api_key() {
        Some(api_key) => api_key,
        None => {
            println!("Error: sender; There is no api key for FCM");
            return Err(Status::InternalServerError);
        }
    };
    // let client = fcm::Client::new();

    unimplemented!()
}
