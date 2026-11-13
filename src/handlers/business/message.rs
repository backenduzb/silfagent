use crate::services::activity::is_online;
use crate::services::greedy::is_greedy;
use crate::{app::state::AppState, db::repositories::messages};
use rustigram::prelude::*;
use rustigram::types::ParseMode;
use rustigram::types::UpdateKind;

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

    match is_online(state).await {
        Ok(is_active) => {
            if !is_active {
                if is_greedy(text).await {
                    ctx.bot
                        .send_message(message.chat.id, &format!("<b>Assalomu alaykum, {}!</b> @python_dev_junior hozir boshqa ish bilan band. Savollaringiz bo'lsa mendan so'rashingiz mumkin.", sender.first_name))
                        .parse_mode(ParseMode::HTML)
                        .business_connection_id(connection_id.to_owned()).await?;
                }
            }
        }

        Err(_) => return Ok(()),
    }
    Ok(())
}
