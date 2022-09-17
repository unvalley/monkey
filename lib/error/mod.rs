use thiserror::Error;

use crate::{eval, lexer::token, parser::ast};

#[derive(Error, Debug, Eq, PartialEq)]
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
    #[error("unknown operator")]
    UnknownOperator {
        operator: ast::Infix,
        left: eval::ObjectType,
        right: eval::ObjectType,
    },
    #[error("type mismatchh")]
    TypeMismatch {
        operator: ast::Infix,
        left: eval::ObjectType,
        right: eval::ObjectType,
    },
}
