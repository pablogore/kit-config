use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration source error: {0}")]
    SourceError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Configuration not found: {0}")]
    NotFound(String),

    #[error("Load error: {0}")]
    Load(String),
}
