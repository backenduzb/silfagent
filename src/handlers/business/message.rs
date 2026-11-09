use crate::richtext::stream;
use crate::{app::state::AppState, db::repositories::messages};
use rustigram::prelude::*;
use rustigram::types::UpdateKind;
use rustigram::types::rich_message::InputRichMessage;

pub async fn handle_message(ctx: Context, state: AppState) -> BotResult<()> {
    let UpdateKind::BusinessMessage(message) = &ctx.update.kind else {
        return Ok(());
    };

    let Some(connection_id) = message.business_connection_id.as_deref() else {
        return Ok(());
    };

    let Some(text) = message.text.as_deref() else {
        return Ok(());
    };
    let Some(sender) = message.from.as_ref() else {
        return Ok(());
    };

    messages::save(
        &state.db,
        message.message_id,
        message.chat.id,
        connection_id.to_owned(),
        text.to_owned(),
        sender.id,
        sender.username.clone(),
        sender.first_name.clone(),
    )
    .await?;

    let response = concat!("<b>Assalomu alaykum!</b>", "\n\n", "<i>from @silf</i>");

    stream::stream_with_business_connection(
        &ctx.bot,
        message.chat.id,
        ctx.update_id().max(1),
        InputRichMessage::from_html(r#"<tg-thinking><i>Javob tayyorlanmoqda...</i></tg-thinking>"#),
        response,
        connection_id,
    )
    .await?;

    Ok(())
}
