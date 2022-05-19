use super::comment::Comment;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentThread {
    pub id: String,
    pub snippet: Snippet,
    pub replies: Option<Replies>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub top_level_comment: Comment,
    pub total_reply_count: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Replies {
    pub comments: Vec<Comment>,
}
