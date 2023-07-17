use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::tokens;

use crate::app::modules::tokens::model::{Token, NewToken};

pub async fn get_all(db: &Db) -> Result<Vec<Token>, diesel::result::Error> {
    let tokens = db
        .run(move |conn| { tokens::table .load::<Token>(conn)
        }).await?;

    Ok(tokens)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Token, diesel::result::Error> {
    let token = db
        .run(move |conn| { tokens::table .find(id) .first::<Token>(conn)
        }).await?;

    Ok(token)
}

pub async fn get_by_user_id(db: &Db, user_id: i32) -> Result<Token, diesel::result::Error> {
    let token = db
        .run(move |conn| { tokens::table.filter(tokens::user_id.eq(user_id)).first::<Token>(conn)
        }).await?;

    Ok(token)
}

pub async fn create(db: &Db, new_token: NewToken) -> Result<Token, diesel::result::Error> {
    let token = db
        .run(move |conn| {
            diesel::insert_into(tokens::table)
                .values(&new_token)
                .get_result::<Token>(conn)
        }).await?;

    Ok(token)
}

pub async fn update(db: &Db, id: i32, token: NewToken) -> Result<Token, diesel::result::Error> {
    let token = db
        .run(move |conn| {
            diesel::update(tokens::table.find(id))
                .set(&token)
                .get_result::<Token>(conn)
        }).await?;

    Ok(token)
}

pub async fn update_by_user_id(db: &Db, user_id: i32, token: NewToken) -> Result<Token, diesel::result::Error> {
    let token = db
        .run(move |conn| {
            diesel::update(tokens::table.filter(tokens::user_id.eq(user_id)))
                .set(&token)
                .get_result::<Token>(conn)
        }).await?;

    Ok(token)
}
