use thiserror::Error;


#[derive(Debug, Error)]
pub enum LcuError {
    #[error("Failed to get LeagueClientUx process")]
    ProcessNotFound,
    #[error("Failed to get the port of LeagueClientUx")]
    PortNotFound,
    #[error("Failed to get the token of LeagueClientUx")]
    TokenNotFound,
}