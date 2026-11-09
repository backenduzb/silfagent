use crate::db::repositories::user::find_by_bussiness_id;
use crate::richtext::stream;
use crate::{app::state::AppState, db::repositories::messages};
use rustigram::prelude::*;
use rustigram::types::UpdateKind;
use rustigram::types::rich_message::InputRichMessage;

pub async fn handle_deleted(ctx: Context, state: AppState) -> BotResult<()> {
    let UpdateKind::DeletedBusinessMessages(deleted) = &ctx.update.kind else {
        return Ok(());
    };

    let mut text = String::new();
    for message_id in &deleted.message_ids {
        if let Some(message) = messages::find(
            &state.db,
            *message_id,
            deleted.chat.id,
            &deleted.business_connection_id,
        )
        .await?
        {
            let username = message.username.as_deref().unwrap_or("username yo'q");
            text.push_str(&format!(
                "<b>O'chirilgan xabar</b><tg-emoji emoji-id=\"5409014699423446355\">🗑️</tg-emoji><blockquote>{}</blockquote><br><b>username</b>: {}\n<br><b>ID</b>: <code>{}</code>\n<b><br>ism</b>: <pre>{}</pre><br><br>#deleted #{}",
                message.text, username, message.telegram_id, message.first_name, username
            ));
        }
    }

    if text.is_empty() {
        text = "O'chirilgan xabar ma'lumotlari oldindan saqlanmagan.".to_owned();
    }
    let Some(admin_user) = find_by_bussiness_id(&state.db, &deleted.business_connection_id).await?
    else {
        return Ok(());
    };

    // let sticker = InputFile::FileId(
    //     "CAACAgIAAxkBAAEkWkRq8ItY0a7_1DlamqGmJ5mnnP7SfAACAUYAAschqUuazloOuy33rj0E".to_string(),
    // );

    // ctx.bot
    //     .send_sticker(deleted.chat.id, sticker)
    //     .business_connection_id(&deleted.business_connection_id)
    //     .await?;

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
