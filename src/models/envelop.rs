use serde::Deserialize;

use super::error::Error;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Envelop<T> {
    pub items: Option<Vec<T>>,
    pub next_page_token: Option<String>,
    pub error: Option<Error>,
}
