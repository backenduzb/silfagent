use crate::config::settings::{CHUNK_SIZE, STREAM_INTERVAL, THINKING_TIME};
use crate::db::repositories::business;
use rustigram::api::methods::ChatAction;
use rustigram::prelude::{BotClient, BotResult};
use rustigram::types::message::ParseMode;
use rustigram::types::rich_message::InputRichMessage;
use sea_orm::DatabaseConnection;
use std::time::Duration;
use tokio::time::Instant;
use tokio::time::sleep;

const DRAFT_ROTATION: Duration = Duration::from_secs(20);
const DRAFT_REFRESH: Duration = Duration::from_secs(2);

pub async fn stream(
    bot: &BotClient,
    chat_id: i64,
    draft_id: i64,
    thinking: InputRichMessage,
    text: &str,
) -> BotResult<()> {
    stream_inner(bot, chat_id, draft_id, thinking, text, None, false).await
}

pub async fn stream_with_business_connection(
    bot: &BotClient,
    chat_id: i64,
    draft_id: i64,
    thinking: InputRichMessage,
    text: &str,
    business_connection_id: &str,
) -> BotResult<()> {
    stream_inner(
        bot,
        chat_id,
        draft_id,
        thinking,
        text,
        Some(business_connection_id),
        true,
    )
    .await
}

async fn stream_inner(
    bot: &BotClient,
    chat_id: i64,
    draft_id: i64,
    thinking: InputRichMessage,
    text: &str,
    business_connection_id: Option<&str>,
    raw_html: bool,
) -> BotResult<()> {
    let mut typing = bot.send_chat_action(chat_id, ChatAction::Typing);
    if let Some(connection_id) = business_connection_id {
        typing = typing.business_connection_id(connection_id);
    }
    typing.await?;

    if business_connection_id.is_none() {
        bot.send_rich_message_draft(chat_id, draft_id, thinking)
            .await?;
    }

    sleep(THINKING_TIME).await;

    let mut full_text = String::new();
    let started = Instant::now();

    for chunk in split_text(text) {
        full_text.push_str(&chunk);

        let current_draft_id = rotating_draft_id(draft_id, started.elapsed());

        let mut typing = bot.send_chat_action(chat_id, ChatAction::Typing);
        if let Some(connection_id) = business_connection_id {
            typing = typing.business_connection_id(connection_id);
        }
        typing.await?;

        if business_connection_id.is_none() {
            bot.send_rich_message_draft(
                chat_id,
                current_draft_id,
                InputRichMessage::from_html(escape_html(&full_text)),
            )
            .await?;
        }

        sleep(STREAM_INTERVAL).await;
    }

    let rendered_text = if raw_html {
        full_text.clone()
    } else {
        escape_html(&full_text)
    };
    let mut response = bot.send_rich_message(chat_id, InputRichMessage::from_html(rendered_text));
    if let Some(connection_id) = business_connection_id {
        response = response.business_connection_id(connection_id);

        if response.await.is_err() {
            bot.send_message(chat_id, text)
                .parse_mode(ParseMode::HTML)
                .business_connection_id(connection_id)
                .await?;
        }
    } else {
        response.await?;
    }

    Ok(())
}

pub async fn wait_for_business_connection(
    bot: &BotClient,
    db: &DatabaseConnection,
    user_id: i64,
    chat_id: i64,
    draft_id: i64,
    timeout: Duration,
) -> BotResult<bool> {
    let deadline = tokio::time::Instant::now() + timeout;
    let started = Instant::now();
    let mut last_refresh = Instant::now() - DRAFT_REFRESH;

    loop {
        if let Some(connection) = business::find_by_user_id(db, user_id).await?
            && connection.is_enabled
        {
            stream(
                bot,
                chat_id,
                draft_id,
                InputRichMessage::from_html(
                    r#"<tg-thinking><i>Checking <tg-emoji emoji-id="5408830295002593386"></tg-emoji></i></tg-thinking>"#,
                ),
                "Business connection ulandi",
            )
            .await?;
            return Ok(true);
        }

        let now = tokio::time::Instant::now();
        if now >= deadline {
            stream(
                bot,
                chat_id,
                draft_id,
                InputRichMessage::from_html(
                    r#"<tg-thinking><i>Vaqti tugadi bye bye <tg-emoji emoji-id="5409196359360197257"></tg-emoji>"></i></tg-thinking>"#,
                ),
                "Business connection topilmadi. Afsus mayli keyinroq sinab ko'rasiz. 😁",
            )
            .await?;
            return Ok(false);
        }

        let remaining_duration = deadline - now;
        let remaining =
            remaining_duration.as_secs() + u64::from(remaining_duration.subsec_nanos() > 0);
        if last_refresh.elapsed() >= DRAFT_REFRESH {
            let current_draft_id = rotating_draft_id(draft_id, started.elapsed());
            bot.send_chat_action(chat_id, ChatAction::Typing).await?;
            bot.send_rich_message_draft(
                chat_id,
                current_draft_id,
                InputRichMessage::from_html(&format!(
                    r#"<tg-thinking><i>Waiting for add profile <tg-emoji emoji-id="5409247280492458305"></tg-emoji> <b>{remaining}s</b></i></tg-thinking>"#
                )),
            )
            .await?;
            last_refresh = Instant::now();
        }

        sleep(Duration::from_secs(1)).await;
    }
}

fn rotating_draft_id(base: i64, elapsed: Duration) -> i64 {
    base.saturating_add((elapsed.as_secs() / DRAFT_ROTATION.as_secs()) as i64)
}

fn split_text(text: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = String::new();

    for part in text.split_inclusive(char::is_whitespace) {
        current.push_str(part);

        if current.chars().count() >= CHUNK_SIZE {
            chunks.push(std::mem::take(&mut current));
        }
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    chunks
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
