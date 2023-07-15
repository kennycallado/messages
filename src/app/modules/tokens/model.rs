use serde::{Deserialize, Serialize};

use crate::database::schema::tokens;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Identifiable)]
#[serde(crate = "rocket::serde")]
pub struct Token{
    pub id: i32,
    pub user_id: i32,
    pub fcm_token: Option<String>,
    pub web_token: Option<rocket::serde::json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Insertable, AsChangeset)]
#[diesel(table_name = tokens)]
#[serde(crate = "rocket::serde")]
pub struct NewToken{
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
