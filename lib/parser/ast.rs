use std::fmt;

/// Root Node for AST
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub struct Node {}

impl Node {
    /// for debug
    pub fn token_literal() {}
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Statement {
    Let {
        /// In Monkeylang, Identifier generates value.
        identifier: Expression,
        value: Expression,
    },
    Return(Expression),
    Expression(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let { identifier, value } => {
                write!(f, "let {:?} = {:?};", identifier, value)
            }
            Statement::Return(value) => write!(f, "return {:?};", value),
            Statement::Expression(value) => write!(f, "{:?};", value),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Expression {
    Identifier(String),
    String(String),
    Integer(i64),
    Prefix { operator: Prefix, right: Box<Expression> },
    Infix { operator: Infix, right: Box<Expression>, left: Box<Expression> }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Prefix {
    Bang,
    Minus,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Infix {
    Eq,
    NotEq,
    LT,
    GT,
    Plus,
    Minus,
    Slash,
    Asterisk
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Precedence {
    Lowest,
    /// == or !=
    Equals,
    /// > or <
    LessGreater,
    /// +
    Sum,
    /// *
    Product,
    /// ! or -
    Prefix,
    /// my_function(x)
    Call,
}

