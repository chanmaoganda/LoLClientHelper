use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("Summoner is not found")]
    SummonerNotFound,
    #[error("History is not found")]
    HistoryNotFound,
}