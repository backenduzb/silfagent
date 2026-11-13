use crate::app::state::AppState;
use reqwest::Error;
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct ActivityApiResponse {
    pub activity: String,
    pub status: String,
}

pub struct Activity {
    pub activity: String,
    pub status: String,
}

pub async fn get_activity(state: AppState) -> Result<Activity, Error> {
    let response: ActivityApiResponse = get(state.activity_url)
        .await?
        .json::<ActivityApiResponse>()
        .await?;

    Ok(Activity {
        activity: response.activity,
        status: response.status,
    })
}

pub async fn is_online(state: AppState) -> Result<bool, Error> {
    match get_activity(state).await {
        Ok(result) => {
            Ok(result.activity == "communication")
        }
        Err(_) => Ok(false),
    }
}