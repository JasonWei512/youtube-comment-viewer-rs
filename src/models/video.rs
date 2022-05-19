use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub snippet: Snippet,
    pub statistics: Statistics,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub title: String,
    pub channel_title: String,
    pub description: String,
    pub published_at: DateTime<Local>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub view_count: String,
    pub like_count: String,
    pub comment_count: String,
}
