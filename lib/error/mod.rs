use thiserror::Error;

use crate::{eval::object, lexer::token, parser::ast};

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
    #[error("identifier not found")]
    IdentifierNotFound,
    #[error("invalid integer")]
    InvalidInteger,
    #[error("unknown error")]
    Unknown,
    #[error("unknown operator")]
    UnknownOperator {
        operator: ast::Infix,
        left: object::ObjectType,
        right: object::ObjectType,
    },
    #[error("type mismatchh")]
    TypeMismatch {
        operator: ast::Infix,
        left: object::ObjectType,
        right: object::ObjectType,
    },
    #[error("incorrect number of arguments")]
    IncorrectNumberOfArguments { expected: usize, actual: usize },
}
