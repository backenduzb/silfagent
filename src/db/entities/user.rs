use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub telegram_id: i64,
    pub username: Option<String>,
    pub first_name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::business::Entity")]
    BusinessConnection,
}

impl Related<super::business::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BusinessConnection.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
