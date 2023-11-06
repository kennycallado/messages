#[cfg(feature = "db_sqlx")]
use rocket_db_pools::sqlx;

use crate::app::modules::messages::model::{NewMessage, Message};
use crate::database::connection::Db;

pub async fn get_all(db: &Db) -> Result<Vec<Message>, sqlx::Error> {
    // let messages = db
    //     .run(move |conn| messages::table.load::<Message>(conn))
    //     .await?;
    let messages = sqlx::query_as!(Message, "SELECT * FROM messages")
        .fetch_all(&db.0)
        .await?;

    Ok(messages)
}

pub async fn get_by_id(db: &Db, id: i32) -> Result<Message, sqlx::Error> {
    // let message = db
    //     .run(move |conn| messages::table.find(id).first::<Message>(conn))
    //     .await?;
    let message = sqlx::query_as!(Message, "SELECT * FROM messages WHERE id = $1", id)
        .fetch_one(&db.0)
        .await?;

    Ok(message)
}

pub async fn create(db: &Db, new_message: NewMessage) -> Result<Message, sqlx::Error> {
    // let message = db
    //     .run(move |conn| {
    //         diesel::insert_into(messages::table)
    //             .values(&new_message)
    //             .get_result::<Message>(conn)
    //     })
    //     .await?;
    let message = sqlx::query_as!(Message, "INSERT INTO messages (title, body, message_type, content, data) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        new_message.title, new_message.body, new_message.message_type, &new_message.content, new_message.data)
        .fetch_one(&db.0)
        .await?;

    Ok(message)
}

pub async fn update(db: &Db, id: i32, new_message: NewMessage) -> Result<Message, sqlx::Error> {
    // let message = db
    //     .run(move |conn| {
    //         diesel::update(messages::table.find(id))
    //             .set(&new_message)
    //             .get_result::<Message>(conn)
    //     })
    //     .await?;
    let message = sqlx::query_as!(Message, "UPDATE messages SET title = $1, body = $2, message_type = $3, content = $4, data = $5 WHERE id = $6 RETURNING *",
        new_message.title, new_message.body, new_message.message_type, &new_message.content, new_message.data, id)
        .fetch_one(&db.0)
        .await?;

    Ok(message)
}
