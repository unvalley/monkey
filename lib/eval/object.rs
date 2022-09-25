use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ObjectType {
    Integer,
    Bool,
    Null,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Object {
    Integer(i64),
    Bool(bool),
    Null,
    Return(Box<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Bool(val) => write!(f, "{}", val),
            Object::Return(val) => write!(f, "{}", val),
            Object::Null => write!(f, "null"),
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
