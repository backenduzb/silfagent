use crate::richtext::stream;
use rustigram::prelude::*;
use rustigram::types::UpdateKind;
use rustigram::types::rich_message::InputRichMessage;

pub async fn handle_message(ctx: Context) -> BotResult<()> {
    let UpdateKind::BusinessMessage(message) = &ctx.update.kind else {
        return Ok(());
    };

    let Some(connection_id) = message.business_connection_id.as_deref() else {
        return Ok(());
    };

    let response = concat!(
        "<b>Assalomu alaykum!</b>",
        
        "\n\n",
        "<i>from @silf</i>"
    );

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
