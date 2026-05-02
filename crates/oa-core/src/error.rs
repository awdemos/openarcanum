use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Character generation failed: {0}")]
    Generation(String),

    #[error("Rule conflict: {0}")]
    RuleConflict(String),

    #[error("System not supported: {0}")]
    UnsupportedSystem(String),

    #[error("Character not found: {0}")]
    CharacterNotFound(String),

    #[error("Invalid attribute value: {0}")]
    InvalidAttribute(String),

    #[error("Level up error: {0}")]
    LevelUp(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Schema validation error: {0}")]
    Schema(String),

    #[error("Storage error: {0}")]
    Storage(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serialization(err.to_string())
    }
}
