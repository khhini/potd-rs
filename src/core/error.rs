use thiserror::Error;

#[derive(Error, Debug)]
pub enum PotdError {
    #[error("Failed to parse Bing API response: Image URL not found {0}")]
    ImageUrlNotFound(String),
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Environment variable error: {0}")]
    Var(#[from] std::env::VarError),
}
