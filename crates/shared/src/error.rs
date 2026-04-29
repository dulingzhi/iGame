use thiserror::Error;

/// Errors that can occur when working with map packages.
#[derive(Error, Debug)]
pub enum MapPackageError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("RON parse error: {0}")]
    Ron(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}
