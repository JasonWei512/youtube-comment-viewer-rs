use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub code: i32,
    pub message: String,
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
