use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Reqwest error: {_0}")]
    ReqwestQueryError(#[from] reqwest::Error),
}