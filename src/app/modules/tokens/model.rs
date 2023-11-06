use serde::{Deserialize, Serialize};

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;

#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub fcm_token: Option<String>,
    pub web_token: Option<rocket::serde::json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NewToken {
    pub user_id: i32,
    pub fcm_token: Option<String>,
    pub web_token: Option<rocket::serde::json::Value>,
}

impl From<Token> for NewToken {
    fn from(token: Token) -> Self{
        Self{
            user_id: token.user_id,
            fcm_token: token.fcm_token,
            web_token: token.web_token,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebPushToken {
    pub endpoint: String,
    #[serde(rename = "expirationTime")]
    pub expiration_time: Option<i32>,
    pub p256dh: String,
    pub auth: String,
}

impl From<&rocket::serde::json::Value> for WebPushToken {
    fn from(value: &rocket::serde::json::Value) -> Self {
        Self {
            endpoint: value["endpoint"].as_str().unwrap().to_string(),
            expiration_time: value["expirationTime"].as_i64().map(|x| x as i32),
            p256dh: value["keys"]["p256dh"].as_str().unwrap().to_string(),
            auth: value["keys"]["auth"].as_str().unwrap().to_string(),
        }
    }
}
