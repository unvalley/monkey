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

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in self.statements.iter() {
            writeln!(f, "{}", stmt)?;
        }
        Ok(())
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
    Prefix {
        operator: Prefix,
        right: Box<Expression>,
    },
    Infix {
        operator: Infix,
        right: Box<Expression>,
        left: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(value) => write!(f, "{}", &value),
            Expression::String(value) => write!(f, "{}", &value),
            Expression::Integer(value) => write!(f, "{}", value),
            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                operator,
                left,
                right,
            } => write!(f, "({}{}{})", left, operator, right),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Prefix {
    Bang,
    Minus,
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Bang => write!(f, "!"),
            Prefix::Minus => write!(f, "-"),
        }
    }
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
    Asterisk,
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Infix::Plus => write!(f, "+"),
            Infix::Minus => write!(f, "-"),
            Infix::Asterisk => write!(f, "*"),
            Infix::Slash => write!(f, "/"),
            Infix::Eq => write!(f, "=="),
            Infix::NotEq => write!(f, "!="),
            Infix::LT => write!(f, "<"),
            Infix::GT => write!(f, ">"),
        }
    }
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
