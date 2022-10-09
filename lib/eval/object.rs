use std::fmt;

use crate::{eval::environment, parser::ast};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ObjectType {
    Integer,
    Bool,
    Null,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Object {
    Integer(i64),
    String(String),
    Bool(bool),
    Null,
    Return(Box<Object>),
    Function {
        parameters: Vec<ast::Expression>,
        body: ast::Statement,
        env: environment::Environment,
    },
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::String(val) => write!(f, "{}", val),
            Object::Bool(val) => write!(f, "{}", val),
            Object::Return(val) => write!(f, "{}", val),
            Object::Null => write!(f, "null"),
            Object::Return(val) => write!(f, "{}", val),
            Object::Function {
                parameters, body, ..
            } => {
                write!(
                    f,
                    "fn({}){{{}}}",
                    parameters
                        .iter()
                        .map(|expr| format!("{}", expr))
                        .collect::<Vec<_>>()
                        .join(","),
                    body
                )
            }
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Null => false,
            Object::Bool(value) => *value,
            _ => true,
        }
    }
}
