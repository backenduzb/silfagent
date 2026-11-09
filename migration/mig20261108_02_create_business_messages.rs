use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BusinessMessages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BusinessMessages::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(BusinessMessages::MessageId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BusinessMessages::ChatId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(BusinessMessages::ConnectionId)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(BusinessMessages::Text).text().not_null())
                    .col(
                        ColumnDef::new(BusinessMessages::TelegramId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(BusinessMessages::Username).string().null())
                    .col(
                        ColumnDef::new(BusinessMessages::FirstName)
                            .string()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .name("uq_business_messages_lookup")
                            .col(BusinessMessages::MessageId)
                            .col(BusinessMessages::ChatId)
                            .col(BusinessMessages::ConnectionId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BusinessMessages::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BusinessMessages {
    Table,
    Id,
    MessageId,
    ChatId,
    ConnectionId,
    Text,
    TelegramId,
    Username,
    FirstName,
}
