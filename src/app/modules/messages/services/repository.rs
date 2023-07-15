use diesel::prelude::*;

use crate::database::connection::Db;
use crate::database::schema::messages;

use crate::app::modules::messages::model::{NewMessage, Message};

pub async fn get_all(db: &Db) -> Result<Vec<Message>, diesel::result::Error> {
    let messages = db
        .run(move |conn| messages::table.load::<Message>(conn))
        .await?;

    Ok(messages)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Message, diesel::result::Error> {
    let message = db
        .run(move |conn| messages::table.find(id).first::<Message>(conn))
        .await?;

    Ok(message)
}

pub async fn create(db: &Db, new_message: NewMessage) -> Result<Message, diesel::result::Error> {
    let message = db
        .run(move |conn| {
            diesel::insert_into(messages::table)
                .values(&new_message)
                .get_result::<Message>(conn)
        })
        .await?;

    Ok(message)
}

pub async fn update(db: &Db, id: i32, new_message: NewMessage) -> Result<Message, diesel::result::Error> {
    let message = db
        .run(move |conn| {
            diesel::update(messages::table.find(id))
                .set(&new_message)
                .get_result::<Message>(conn)
        })
        .await?;

    Ok(message)
}
