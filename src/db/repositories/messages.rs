use crate::db::entities::messages::{ActiveModel, Column, Entity, Model};
use anyhow::Result;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn save(
    db: &DatabaseConnection,
    message_id: i64,
    chat_id: i64,
    connection_id: String,
    text: String,
    telegram_id: i64,
    username: Option<String>,
    first_name: String,
) -> Result<()> {
    Entity::insert(ActiveModel {
        message_id: Set(message_id),
        chat_id: Set(chat_id),
        connection_id: Set(connection_id),
        text: Set(text),
        telegram_id: Set(telegram_id),
        username: Set(username),
        first_name: Set(first_name),
        ..Default::default()
    })
    .exec(db)
    .await?;
    Ok(())
}

pub async fn find(
    db: &DatabaseConnection,
    message_id: i64,
    chat_id: i64,
    connection_id: &str,
) -> Result<Option<Model>> {
    Ok(Entity::find()
        .filter(Column::MessageId.eq(message_id))
        .filter(Column::ChatId.eq(chat_id))
        .filter(Column::ConnectionId.eq(connection_id))
        .one(db)
        .await?)
}
