use serde::{Deserialize, Serialize};

#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx::FromRow;

#[cfg_attr(feature = "db_sqlx", derive(FromRow))]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub message_type: String,
    pub content: Vec<String>,
    pub data: rocket::serde::json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct NewMessage {
    pub title: String,
    pub body: Option<String>,
    pub message_type: Option<String>,
    pub content: Vec<String>,
    pub data: Option<rocket::serde::json::Value>,
}

impl From<Message> for NewMessage {
    fn from(message: Message) -> Self {
        NewMessage {
            title: message.title,
            body: message.body,
            message_type: Some(message.message_type),
            content: message.content,
            data: Some(message.data),
        }
    }
}

impl From<NewMessage> for rocket::serde::json::Value {
    fn from(message: NewMessage) -> Self {
        rocket::serde::json::json!({
            "notification": {
                "title": message.title,
                "body": message.body,
                "vibrate": [100, 50, 100],
                "data": rocket::serde::json::json!({
                    "type": message.message_type,
                    "content": message.content,
                }),
            }
        })
    }
}
