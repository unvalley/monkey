/// Root Node for AST
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
    // fn token_literal(&mut self) -> String {
    //     if self.statements.len() > 0 {
    //         self.statements[0].token_literal()
    //     } else {
    //         String::new()
    //     }
    // }
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
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Expression {
    Identifier(String),
    String(String),
    Integer(i64),
}
