use thiserror::Error;

use crate::lexer::token;

#[derive(Error, Debug)]
pub enum MonkeyError {
    #[error("invalid token")]
    InvalidToken(token::Token),
    #[error("invalid identifier")]
    InvalidIdentifier,
    #[error("invalid integer")]
    InvalidInteger,
    #[error("unknown error")]
    Unknown,
}
