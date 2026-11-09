use crate::app::state::AppState;
use crate::db::repositories::user::find_by_bussiness_id;
use crate::richtext::stream;
use rustigram::prelude::*;
use rustigram::types::UpdateKind;
use rustigram::types::rich_message::InputRichMessage;

pub async fn handle_edited(ctx: Context, state: AppState) -> BotResult<()> {
    let UpdateKind::EditedBusinessMessage(message) = &ctx.update.kind else {
        return Ok(());
    };

    let connection_id = match &message.business_connection_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let Some(admin_user) = find_by_bussiness_id(&state.db, &connection_id).await? else {
        return Ok(());
    };

    let msgtext = match &message.text {
        Some(text) => text,
        None => return Ok(()),
    };

    let text = &format!(
        "<b>Tahrirlangan habar topildi</b><tg-emoji emoji-id=\"5409014699423446355\">🗑️</tg-emoji><blockquote>{}</blockquote><br><b>username</b>: {}\n<br><b>ID</b>: <code>{}</code>\n<b><br>ism</b>: <pre>{}</pre><br><br>#edited #{}",
        msgtext,
        message.chat.username.as_deref().unwrap_or("null"),
        message.chat.id,
        message.chat.first_name.as_deref().unwrap_or("null"),
        message.chat.username.as_deref().unwrap_or("null"),
    );

    stream::stream_html(
        &ctx.bot,
        admin_user.telegram_id,
        ctx.update_id().max(1),
        InputRichMessage::from_html(
            r#"<tg-thinking><i>Colllecting information ...</i></tg-thinking>"#,
        ),
        &text,
    )
    .await?;

    Ok(())
}
