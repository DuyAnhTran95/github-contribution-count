use thiserror::Error;

#[derive(Error, Debug)]
pub enum GithubError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}