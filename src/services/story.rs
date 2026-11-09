use reqwest::multipart::{Form, Part};
use rustigram::api::error::Error;
use rustigram::prelude::{BotClient, BotResult};
use rustigram::types::story::{InputStoryContent, InputStoryContentVideo, Story};
use std::path::Path;

pub async fn post_story_video(
    bot: &BotClient,
    business_connection_id: &str,
    video_file_id: &str,
    duration: u32,
    caption: Option<&str>,
) -> BotResult<()> {
    let file = bot.get_file(video_file_id).await?;
    let file_path = file.file_path.ok_or(Error::MissingParam("file_path"))?;
    let data = bot.download_file(&file_path).await?;

    let extension = Path::new(&file_path)
        .extension()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("mp4");
    let mime = match extension.to_ascii_lowercase().as_str() {
        "mp4" => "video/mp4",
        "mov" => "video/quicktime",
        "webm" => "video/webm",
        _ => "application/octet-stream",
    };

    let content = InputStoryContent::Video(InputStoryContentVideo {
        video: "attach://story_media".to_owned(),
        duration: (1..=60).contains(&duration).then_some(f64::from(duration)),
        cover_frame_timestamp: None,
        is_animation: None,
    });
    let content = serde_json::to_string(&content).map_err(Error::from)?;

    let media = Part::bytes(data.to_vec())
        .file_name(format!("story.{extension}"))
        .mime_str(mime)
        .map_err(Error::from)?;

    let mut form = Form::new()
        .text("business_connection_id", business_connection_id.to_owned())
        .text("content", content)
        .text("active_period", "86400")
        .part("story_media", media);

    if let Some(caption) = caption {
        form = form.text("caption", caption.to_owned());
    }

    bot.post_multipart::<Story>("postStory", form).await?;

    Ok(())
}
