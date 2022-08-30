#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal,
    EOF,
    Ident(String),
    StringLiteral(String),
    IntLiteral(i64),
    BoolLitral(bool),
    Assign,
    Plus,
    Comma,
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
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Tokens<'a> {
    pub tok: &'a [Token],
}
