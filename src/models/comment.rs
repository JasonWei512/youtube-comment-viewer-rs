use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub snippet: CommentSnippet,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommentSnippet {
    pub author_display_name: String,
    pub published_at: DateTime<Local>,
    pub like_count: u32,
    pub text_display: String,
}
