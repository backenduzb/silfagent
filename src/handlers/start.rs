use crate::app::state::AppState;
use crate::db::repositories::business;
use crate::db::repositories::user;
use crate::richtext::stream;
use rustigram::prelude::*;
use rustigram::types::rich_message::InputRichMessage;
use std::time::Duration;

pub async fn start(ctx: Context, state: AppState) -> BotResult<()> {
    let Some(ChatId::Id(chat_id)) = ctx.chat_id() else {
        return Ok(());
    };

    let Some(message) = ctx.message() else {
        return Ok(());
    };

    let Some(telegram_user) = message.from.as_ref() else {
        return Ok(());
    };

    let user = user::upsert_profile(
        &state.db,
        telegram_user.id,
        telegram_user.username.clone(),
        telegram_user.first_name.clone(),
    )
    .await?;
    let connection = business::find_by_user_id(&state.db, user.id).await?;

    match connection {
        Some(connection) if connection.is_enabled => {
            stream::stream(
                &ctx.bot,
                chat_id,
                ctx.update_id().max(1),
                InputRichMessage::from_html("<tg-thinking><i>Identifing...</i></tg-thinking>"),
                &format!("Assalomu alaykum, {}!", user.first_name),
            )
            .await?;
        }
        _ => {
            stream::stream(
                &ctx.bot,
                chat_id,
                ctx.update_id().max(1),
                InputRichMessage::from_html("<tg-thinking><i>Identifing...</i></tg-thinking>"),
                &format!("Assalomu alaykum, {}!", telegram_user.first_name),
            )
            .await?;

            stream::wait_for_business_connection(
                &ctx.bot,
                &state.db,
                user.id,
                chat_id,
                ctx.update_id().max(1),
                Duration::from_secs(120),
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn start_with_state(ctx: Context, _state: AppState) -> BotResult<()> {
    start(ctx, _state).await
}
