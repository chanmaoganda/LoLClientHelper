use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Reqwest Query error: {_0}")]
    ReqwestQueryError(#[from] reqwest::Error),
    #[error("Json Parse error: {_0}")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Unpack error: {_0}")]
    UnpackError(String),
}