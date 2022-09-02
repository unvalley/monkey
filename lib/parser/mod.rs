use crate::{
    error::MonkeyError,
    lexer::{token, Lexer},
};

pub mod ast;

#[derive(Debug, Clone)]
pub struct Parser {
    l: Lexer,
    current_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(l: Lexer) -> Parser {
        let mut p = Parser {
            l,
            current_token: token::Token::Illegal,
            peek_token: token::Token::Illegal,
        };
        // read two tokens
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.l.next_token();
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, MonkeyError> {
        let mut program = ast::Program::new();
        while !self.is_current_token(token::Token::EOF) {
            let stmt = self.parse_statement()?;
            program.statements.push(stmt);
            self.next_token()
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        match self.current_token {
            token::Token::Let => self.parse_let_statement(),
            _ => self.parse_let_statement()
            // _ => Err(MonkeyError::Unknown) // delete soon
            // _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        // current_token is token::Token::Let
        self.next_token();

        let identifier = if let token::Token::Ident(ident) = &self.current_token {
            ast::Expression::Identifier(ident.to_owned())
        } else {
            // TODO:
            return Err(MonkeyError::Unknown);
        };

        self.expect_peek(token::Token::Assign)?;
        self.next_token();

        // let value = self.parse_expression()?;
        let stmt = ast::Statement::Let {
            identifier,
            value: ast::Expression::Identifier("temp".to_string()),
        };
        Ok(stmt)
    }

    fn parse_expression(&mut self) -> Result<ast::Expression, MonkeyError> {
        Err(MonkeyError::Unknown)
    }

    fn is_current_token(&mut self, token: token::Token) -> bool {
        self.current_token == token
    }

    fn is_peek_token(&mut self, token: token::Token) -> bool {
        self.peek_token == token
    }

    fn expect_peek(&mut self, token: token::Token) -> Result<(), MonkeyError> {
        if self.is_peek_token(token.clone()) {
            self.next_token();
            Ok(())
        } else {
            Err(MonkeyError::InvalidToken(token))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::Program;

    use super::*;

    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        // doesn't mock lexer for readability
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();

        if program.statements.len() != 3 {
            panic!(
                "program.statements should be 3. but got {}",
                program.statements.len()
            );
        }

        let expected_identifier = vec!["x", "y", "foobar"];
        for (i, ident) in expected_identifier.iter().enumerate() {
            let_statement(&program.statements[i], ident)
        }
    }

    fn let_statement(stmt: &ast::Statement, name: &str) {
        if let ast::Statement::Let { identifier, .. } = stmt {
            if let ast::Expression::Identifier(identifier) = identifier {
                assert_eq!(identifier, name)
            } else {
                panic!("expected ast::Statement::Let, but got {:?}", stmt);
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "
        return 5;
        return 10;
        return 993322;
        ";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program  = p.parse_program().unwrap();
        if program.statements.len() != 3 {
            panic!(
                "program.statements should be 3. but got {}",
                program.statements.len()
            );
        }
        for stmt in program.statements.iter() {
            if let ast::Statement::Return(_) = stmt {
            } else {
                panic!("expected ast::Statement::Return, but got {:?}", stmt);
            }
        }
    }
}
