use derive_more::{Display, From};
use secret_rpc::error::ParseError;
use secret_rpc::Error as SecretError;
use tokio::task::JoinError;

#[derive(Display, From, Debug)]
pub enum Error {
    NotFound,
    ChainError(SecretError),
    ParseError(ParseError),
    ThreadError(JoinError),
    SerdeJsonError(serde_json::Error),
    BatchError(String),
    Base64Error(base64::DecodeError),
}

impl std::error::Error for Error {}