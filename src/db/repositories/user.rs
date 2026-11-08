use crate::db::entities::user::{ActiveModel, Column, Entity, Model};
use anyhow::Result;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    sea_query::OnConflict,
};

pub async fn find_by_telegram_id(
    db: &DatabaseConnection,
    telegram_id: i64,
) -> Result<Option<Model>> {
    let user = Entity::find()
        .filter(Column::TelegramId.eq(telegram_id))
        .one(db)
        .await?;

    Ok(user)
}

pub async fn upsert_profile(
    db: &DatabaseConnection,
    telegram_id: i64,
    username: Option<String>,
    first_name: String,
) -> Result<Model> {
    let active_model = ActiveModel {
        telegram_id: Set(telegram_id),
        username: Set(username),
        first_name: Set(first_name),
        ..Default::default()
    };

    Entity::insert(active_model)
        .on_conflict(
            OnConflict::column(Column::TelegramId)
                .update_columns([Column::Username, Column::FirstName])
                .to_owned(),
        )
        .exec(db)
        .await?;

    let user = find_by_telegram_id(db, telegram_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found after upsert"))?;

    Ok(user)
}
