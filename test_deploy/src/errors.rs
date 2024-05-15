use starknet::core::types::contract::ComputeClassHashError;
use starknet::core::types::FromStrError;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("failed to parse url")]
    ParsingError(#[from] ParseError),

    #[error("Database error: {0}")]
    FromStrError(#[from] FromStrError),

    #[error("Database error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    ReadFileError(#[from] std::io::Error),
    #[error("Database error: {0}")]
    JsonError(#[from] starknet::core::types::contract::JsonError),
    #[error("Database error: {0}")]
    ClassHashError(#[from] ComputeClassHashError),
}
