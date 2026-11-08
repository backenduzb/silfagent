use anyhow::Result;
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    sea_query::OnConflict,
};

use crate::db::entities::business::{ActiveModel, Column, Entity, Model};

pub async fn find_by_user_id(db: &DatabaseConnection, user_id: i64) -> Result<Option<Model>> {
    let connection = Entity::find()
        .filter(Column::UserId.eq(user_id))
        .one(db)
        .await?;

    Ok(connection)
}

pub async fn upsert(
    db: &DatabaseConnection,
    user_id: i64,
    connection_id: String,
    is_enabled: bool,
) -> Result<Model> {
    let active_model = ActiveModel {
        user_id: Set(user_id),
        connection_id: Set(connection_id),
        is_enabled: Set(is_enabled),
        ..Default::default()
    };

    Entity::insert(active_model)
        .on_conflict(
            OnConflict::column(Column::UserId)
                .update_columns([Column::ConnectionId, Column::IsEnabled])
                .to_owned(),
        )
        .exec(db)
        .await?;

    let connection = find_by_user_id(db, user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Business connection not found after upsert"))?;

    Ok(connection)
}
