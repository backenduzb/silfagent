use crate::app::state::{AppState, PendingStory, StoryStage};
use crate::db::repositories::{business, user};
use crate::keyboards;
use crate::richtext::stream;
use crate::services::story as story_service;
use rustigram::api::methods::ChatAction;
use rustigram::prelude::*;
use rustigram::types::UpdateKind;
use rustigram::types::keyboard::ReplyMarkup;
use rustigram::types::rich_message::InputRichMessage;
use std::time::Duration;

async fn prompt(
    ctx: &Context,
    chat_id: i64,
    connection_id: Option<&str>,
    text: &str,
    markup: ReplyMarkup,
) -> BotResult<()> {
    let mut request = ctx
        .bot
        .send_rich_message(chat_id, InputRichMessage::from_html(text));
    if let Some(connection_id) = connection_id {
        request = request.business_connection_id(connection_id);
    }
    request.reply_markup(markup).await?;
    Ok(())
}

pub async fn handle_story(ctx: Context, state: AppState) -> BotResult<()> {
    let UpdateKind::Message(message) = &ctx.update.kind else {
        return Ok(());
    };
    let Some(sender) = message.from.as_ref() else {
        return Ok(());
    };

    let Some(profile) = user::find_by_telegram_id(&state.db, sender.id).await? else {
        return Ok(());
    };
    let Some(connection) = business::find_by_user_id(&state.db, profile.id).await? else {
        return Ok(());
    };
    if !connection.is_enabled {
        return Ok(());
    }
    let connection_id = connection.connection_id;
    let key = (connection_id.to_owned(), message.chat.id);

    let video = message
        .video
        .as_ref()
        .map(|video| (video.file_id.clone(), video.duration))
        .or_else(|| {
            message.document.as_ref().and_then(|document| {
                document
                    .mime_type
                    .as_deref()
                    .filter(|mime| mime.starts_with("video/"))
                    .map(|_| (document.file_id.clone(), 0))
            })
        });

    if let Some((video_file_id, duration)) = video {
        let pending = PendingStory {
            video_file_id,
            duration,
            caption: message.caption.clone(),
            stage: if message.caption.is_some() {
                StoryStage::CaptionChoice
            } else {
                StoryStage::CaptionInput
            },
        };
        state.pending_stories.write().await.insert(key, pending);

        stream::stream_thinking(
            &ctx.bot,
            message.chat.id,
            ctx.update_id().max(1),
            InputRichMessage::from_html(
                r#"<tg-thinking><i>Checking video information <tg-emoji emoji-id="5408830295002593386"></tg-emoji></i></tg-thinking>"#,
            ),
            Duration::from_secs(2),
            None,
        )
        .await?;

        if message.caption.is_some() {
            prompt(
                &ctx,
                message.chat.id,
                None,
                "Video captioni bor. Shu captionni ishlataymi?",
                keyboards::yes_no(),
            )
            .await?;
        } else {
            prompt(
                &ctx,
                message.chat.id,
                None,
                "Video uchun caption yozing:",
                keyboards::remove(),
            )
            .await?;
        }
        return Ok(());
    }

    let Some(mut pending) = state.pending_stories.write().await.remove(&key) else {
        return Ok(());
    };
    let answer = message.text.as_deref().unwrap_or_default().trim();

    match pending.stage {
        StoryStage::CaptionChoice if answer.eq_ignore_ascii_case("Ha") => {
            pending.stage = StoryStage::UploadChoice;
            state.pending_stories.write().await.insert(key, pending);
            ask_upload(&ctx, message.chat.id).await?;
        }
        StoryStage::CaptionChoice if answer.eq_ignore_ascii_case("Yo'q") => {
            pending.caption = None;
            pending.stage = StoryStage::CaptionInput;
            state.pending_stories.write().await.insert(key, pending);
            prompt(
                &ctx,
                message.chat.id,
                None,
                "Yangi captionni yozib yuboring:",
                keyboards::remove(),
            )
            .await?;
        }
        StoryStage::CaptionInput => {
            if answer.is_empty() {
                state.pending_stories.write().await.insert(key, pending);
                prompt(
                    &ctx,
                    message.chat.id,
                    None,
                    "Caption bo'sh bo'lmasin. Yangi captionni yozing:",
                    keyboards::remove(),
                )
                .await?;
            } else {
                pending.caption = Some(answer.to_owned());
                pending.stage = StoryStage::UploadChoice;
                state.pending_stories.write().await.insert(key, pending);
                ask_upload(&ctx, message.chat.id).await?;
            }
        }
        StoryStage::UploadChoice if answer.eq_ignore_ascii_case("Ha") => {
            if let Err(error) = upload_story(&ctx, &connection_id, message.chat.id, &pending).await
            {
                state.pending_stories.write().await.insert(key, pending);
                let text = format!(
                    "Story upload qilinmadi: {}\nQayta urinaymi?",
                    stream::escape_html(&error.to_string())
                );
                prompt(&ctx, message.chat.id, None, &text, keyboards::yes_no()).await?;
            }
        }
        StoryStage::UploadChoice if answer.eq_ignore_ascii_case("Yo'q") => {
            prompt(
                &ctx,
                message.chat.id,
                None,
                "Upload bekor qilindi.",
                keyboards::remove(),
            )
            .await?;
        }
        _ => {
            state.pending_stories.write().await.insert(key, pending);
            prompt(
                &ctx,
                message.chat.id,
                None,
                "Iltimos, Ha yoki Yo'q tugmasini bosing.",
                keyboards::yes_no(),
            )
            .await?;
        }
    }

    Ok(())
}

async fn upload_story(
    ctx: &Context,
    connection_id: &str,
    chat_id: i64,
    pending: &PendingStory,
) -> BotResult<()> {
    let upload = story_service::post_story_video(
        &ctx.bot,
        connection_id,
        &pending.video_file_id,
        pending.duration,
        pending.caption.as_deref(),
    );

    stream::stream_while(
        &ctx.bot,
        chat_id,
        ctx.update_id().max(1),
        InputRichMessage::from_html(
            r#"<tg-thinking><i>Uploading your story <tg-emoji emoji-id="5408830295002593386"></tg-emoji></i></tg-thinking>"#,
        ),
        ChatAction::UploadVideo,
        None,
        upload,
        "<b>Upload qilindi.</b>",
    )
    .await
}

async fn ask_upload(ctx: &Context, chat_id: i64) -> BotResult<()> {
    prompt(
        ctx,
        chat_id,
        None,
        "Shu caption bilan storyga upload qilaymi?",
        keyboards::yes_no(),
    )
    .await
}
