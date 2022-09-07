use std::fmt::{self, write};

use thiserror::Error;

use crate::lexer::token;

#[derive(Error, Debug)]
pub enum MonkeyError {
    #[error("unexpected token")]
    UnexpectedToken {
        expected: token::Token,
        actual: token::Token,
    },
    #[error("invalid token")]
    InvalidToken(token::Token),
    #[error("invalid identifier")]
    InvalidIdentifier,
    #[error("invalid integer")]
    InvalidInteger,
    #[error("unknown error")]
    Unknown,
}
