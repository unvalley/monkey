use crate::parser::ast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Illegal,
    EOF,

    // identifier and literal
    Identifier(String),
    StringLiteral(String),
    IntLiteral(i64),
    BoolLitral(bool),
    /// =
    Assign,
    /// +
    Plus,
    /// -
    Minus,
    /// !
    Bang,
    /// *
    Asterisk,
    /// /
    Slash,

    /// <
    LT,
    /// >
    GT,

    // delimiter
    /// ,
    Comma,
    /// :
    Colon,
    /// ;
    SemiColon,

    // punctuations
    /// "("
    LParen,
    /// ")"
    RParen,
    /// "{"
    LBrace,
    /// "}"
    RBrace,
    // reserved
    Function,
    Let,
    Return,
    True,
    False,
    If,
    Else,

    Eq,
    NotEq,
}

impl Token {
    pub fn precedence(&self) -> ast::Precedence {
        match self {
            Token::Eq => ast::Precedence::Equals,
            Token::NotEq => ast::Precedence::Equals,
            Token::LT => ast::Precedence::LessGreater,
            Token::GT => ast::Precedence::LessGreater,
            Token::Plus => ast::Precedence::Sum,
            Token::Minus => ast::Precedence::Sum,
            Token::Asterisk => ast::Precedence::Product,
            Token::Slash => ast::Precedence::Product,
            Token::LParen => ast::Precedence::Call,
            _ => ast::Precedence::Lowest
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tokens<'a> {
    pub tok: &'a [Token],
}
