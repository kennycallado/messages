#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::tokens::model::{NewToken, Token};
use crate::database::connection::Db;

pub async fn get_all(db: &Db) -> Result<Vec<Token>, sqlx::Error> {
    // let tokens = db
    //     .run(move |conn| { tokens::table .load::<Token>(conn)
    //     }).await?;
    let tokens = sqlx::query_as!(Token, "SELECT * FROM tokens")
        .fetch_all(&db.0)
        .await?;

    Ok(tokens)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Token, sqlx::Error> {
    // let token = db
    //     .run(move |conn| { tokens::table .find(id) .first::<Token>(conn)
    //     }).await?;
    let token = sqlx::query_as!(Token, "SELECT * FROM tokens WHERE id = $1", id)
        .fetch_one(&db.0)
        .await?;

    Ok(token)
}

pub async fn get_by_user_id(db: &Db, user_id: i32) -> Result<Token, sqlx::Error> {
    // let token = db
    //     .run(move |conn| { tokens::table.filter(tokens::user_id.eq(user_id)).first::<Token>(conn)
    //     }).await?;
    let token = sqlx::query_as!(Token, "SELECT * FROM tokens WHERE user_id = $1", user_id)
        .fetch_one(&db.0)
        .await?;

    Ok(token)
}

pub async fn create(db: &Db, new_token: NewToken) -> Result<Token, sqlx::Error> {
    // let token = db
    //     .run(move |conn| {
    //         diesel::insert_into(tokens::table)
    //             .values(&new_token)
    //             .get_result::<Token>(conn)
    //     }).await?;
    let token = sqlx::query_as!(
        Token,
        "INSERT INTO tokens (user_id, fcm_token, web_token) VALUES ($1, $2, $3) RETURNING *",
        new_token.user_id,
        new_token.fcm_token,
        new_token.web_token
    )
    .fetch_one(&db.0)
    .await?;

    Ok(token)
}

pub async fn update(db: &Db, id: i32, token: NewToken) -> Result<Token, sqlx::Error> {
    // let token = db
    //     .run(move |conn| {
    //         diesel::update(tokens::table.find(id))
    //             .set(&token)
    //             .get_result::<Token>(conn)
    //     }).await?;
    let token = sqlx::query_as!(Token, "UPDATE tokens SET user_id = $1, fcm_token = $2, web_token = $3 WHERE id = $4 RETURNING *", token.user_id, token.fcm_token, token.web_token, id)
        .fetch_one(&db.0)
        .await?;

    Ok(token)
}

pub async fn update_by_user_id(
    db: &Db,
    user_id: i32,
    token: NewToken,
) -> Result<Token, sqlx::Error> {
    // let token = db
    //     .run(move |conn| {
    //         diesel::update(tokens::table.filter(tokens::user_id.eq(user_id)))
    //             .set(&token)
    //             .get_result::<Token>(conn)
    //     }).await?;
    let token = sqlx::query_as!(Token, "UPDATE tokens SET user_id = $1, fcm_token = $2, web_token = $3 WHERE user_id = $4 RETURNING *", token.user_id, token.fcm_token, token.web_token, user_id)
        .fetch_one(&db.0)
        .await?;

    Ok(token)
}
