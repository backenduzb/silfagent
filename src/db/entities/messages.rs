use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "business_messages")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub message_id: i64,
    pub chat_id: i64,
    pub connection_id: String,
    pub text: String,
    pub telegram_id: i64,
    pub username: Option<String>,
    pub first_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl ActiveModelBehavior for ActiveModel {}
