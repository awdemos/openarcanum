use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum SdkError {
    #[error("HTTP request failed: {0}")]
    Http(String),

    #[error("Serialization failed: {0}")]
    Serialization(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Not found")]
    NotFound,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

impl From<reqwest::Error> for SdkError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_status() {
            if let Some(status) = err.status() {
                if status == reqwest::StatusCode::NOT_FOUND {
                    return SdkError::NotFound;
                }
            }
        }
        SdkError::Http(err.to_string())
    }
}

impl From<serde_json::Error> for SdkError {
    fn from(err: serde_json::Error) -> Self {
        SdkError::Serialization(err.to_string())
    }
}
