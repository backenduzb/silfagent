use sea_orm::DatabaseConnection;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct PendingStory {
    pub video_file_id: String,
    pub duration: u32,
    pub caption: Option<String>,
    pub stage: StoryStage,
}

#[derive(Clone, Debug)]
pub enum StoryStage {
    CaptionChoice,
    CaptionInput,
    UploadChoice,
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub activity_url: String,
    pub pending_stories: Arc<RwLock<HashMap<(String, i64), PendingStory>>>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, activity_url: String) -> Self {
        Self {
            db,
            activity_url,
            pending_stories: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
