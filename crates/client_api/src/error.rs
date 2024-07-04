use thiserror::Error;

#[derive(Debug, Error)]
pub enum QueryError {
    #[error("Reqwest error: {_0}")]
    Reqwest(#[from] reqwest::Error),
}