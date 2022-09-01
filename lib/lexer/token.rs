#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,

    // identifier and literal
    Ident(String),
    StringLiteral(String),
    IntLiteral(i64),
    BoolLitral(bool),
    // 
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    /// <
    LT,
    /// >
    GT,

    // delimiter
    Comma,
    Colon,
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
    NotEq
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tokens<'a> {
    pub tok: &'a [Token],
}
