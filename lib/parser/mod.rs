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
            token::Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        // current_token is token::Token::Let
        self.next_token();
        let identifier = if let token::Token::Identifier(ident) = &self.current_token {
            ast::Expression::Identifier(ident.to_owned())
        } else {
            return Err(MonkeyError::InvalidToken(self.current_token.clone()));
        };
        self.expect_peek(token::Token::Assign)?;
        self.next_token();
        Ok(ast::Statement::Let {
            identifier,
            value: self.parse_expression(ast::Precedence::Lowest)?
        })
    }

    fn parse_return_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        self.next_token();
        let value = self.parse_expression(ast::Precedence::Lowest)?;
        self.expect_peek(token::Token::SemiColon)?;
        Ok(ast::Statement::Return(value))
    }

    fn parse_expression_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        // 優先順位について何の知識もないのでLowestを渡す
        let expr = self.parse_expression(ast::Precedence::Lowest)?;
        if self.is_peek_token(token::Token::SemiColon) {
            self.next_token();
        }
        Ok(ast::Statement::Expression(expr))
    }

    // precedenceの値は呼び出し側で把握している情報と文脈によって変化する．
    fn parse_expression(&mut self, precedence: ast::Precedence) -> Result<ast::Expression, MonkeyError> {
        let mut left_exp = match &self.current_token {
            token::Token::Identifier(ident) => ast::Expression::Identifier(ident.to_owned()),
            token::Token::StringLiteral(str) => ast::Expression::String(str.to_owned()),
            token::Token::IntLiteral(int) => ast::Expression::Integer(*int),
            token::Token::Bang => self.parse_prefix_expression()?,
            token::Token::Minus => self.parse_prefix_expression()?,
            _ => return Err(MonkeyError::InvalidToken(self.current_token.clone()))
        };

        // 中間演算子の処理
        while !self.is_peek_token(token::Token::SemiColon) && precedence < self.peek_precedence() {
            match self.peek_token {
                token::Token::Plus => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                },
                token::Token::Minus => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                },
                token::Token::Slash => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                },
                token::Token::Asterisk => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                },
                token::Token::Eq => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                },
                token::Token::NotEq => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                },
                token::Token::LT => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                },
                token::Token::GT => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                },
                // TODO: LParen
                _ => {return Ok(left_exp)}
            }
        }
        Ok(left_exp)
    }

    /// 開始：前置演算子のtokenがself.current_tokenにセットされた状態．
    /// 終了：前置演算子式のオペランドの最後のtokenがcurrent_tokenにセットされた状態．
    fn parse_prefix_expression(&mut self) -> Result<ast::Expression, MonkeyError> {
        let op = match self.current_token {
            token::Token::Bang => ast::Prefix::Bang,
            token::Token::Minus => ast::Prefix::Minus,
            _ => {
                return Err(MonkeyError::InvalidToken(self.current_token.clone()))
            }
        };
        self.next_token();
        // 優先順位としてPrefix渡す．なぜならこの関数が前置演算子式をparseしている最中だから
        let right = self.parse_expression(ast::Precedence::Prefix)?;
        Ok(ast::Expression::Prefix { operator: op, right: Box::new(right) })
    }

    fn parse_infix_expression(
        &mut self,
        left_expression: ast::Expression,
    ) -> Result<ast::Expression, MonkeyError> {
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

    fn current_precedence(&mut self) -> ast::Precedence {
        self.current_token.precedence()
    }

    fn peek_precedence(&mut self) -> ast::Precedence {
        self.peek_token.precedence()
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(program.statements.len(), 3);
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
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements.len(), 3);
        for stmt in program.statements.iter() {
            if let ast::Statement::Return(_) = stmt {
            } else {
                panic!("expected ast::Statement::Return, but got {:?}", stmt);
            }
        }
    }

    #[test]
    fn test_identifier_expresion() {
        // In Monkeylang, identifier is expression.
        let input = "foobar;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        if let ast::Statement::Expression(expr) = stmt {
            if let ast::Expression::Identifier(ident) = expr {
                assert_eq!(ident, "foobar")
            } else {
                panic!("Incorrect expression");
            }
        } else {
            panic!("Incorrect statement");
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        if let ast::Statement::Expression(expr) = stmt {
            if let ast::Expression::Integer(ident) = expr {
                assert_eq!(ident, &5);
            } else {
                panic!("Incorrect expression");
            }
        } else {
                panic!("Incorrect statement");
        }
    }

    #[test]
    fn test_parse_prefix_expressions() {
        let prefix_tests = vec![
            maplit::hashmap!("input" => "!5;", "operator" => "!", "integer_value" => "5"),
            maplit::hashmap!("input" => "-15;", "operator" => "-", "integer_value" => "15"),
        ];
        for test in prefix_tests.iter() {
            let l = Lexer::new(test.get("input").unwrap().to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            assert_eq!(program.statements.len(), 1);
            // REFACTOR:
            let op = format!("{}", &program.statements[0]);
            assert_eq!(op.as_str(), *test.get("operator").unwrap())
        }
    }

    #[test]
    fn test_parse_infix_expressions() {
        let infix_tests = vec![
            maplit::hashmap!("input" => "5+5;", "left" => "5", "operator" => "+", "right" => "5"),
            maplit::hashmap!("input" => "5-5;", "left" => "5", "operator" => "-", "right" => "5"),
            maplit::hashmap!("input" => "5*5;", "left" => "5", "operator" => "*", "right" => "5"),
            maplit::hashmap!("input" => "5/5;", "left" => "5", "operator" => "/", "right" => "5"),
            maplit::hashmap!("input" => "5>5;", "left" => "5", "operator" => ">", "right" => "5"),
            maplit::hashmap!("input" => "5<5;", "left" => "5", "operator" => "<", "right" => "5"),
            maplit::hashmap!("input" => "5==5;", "left" => "5", "operator" => "==", "right" => "5"),
            maplit::hashmap!("input" => "5!=5;", "left" => "5", "operator" => "!=", "right" => "5"),
        ];
        for test in infix_tests.iter() {
            let l = Lexer::new(test.get("input").unwrap().to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            assert_eq!(program.statements.len(), 1);
            let expr = match &program.statements[0] {
                ast::Statement::Expression(expr) => expr,
                _ => panic!("programs.statements[0] should be Expression but got {:?}", &program.statements[0])
            };
            let infix = match expr {
                ast::Expression::Infix { operator, right: _, left: _ } => operator,
                _ => panic!("")
            };
            let actual = format!("{:?}", infix);
            let expected  = *test.get("operator").unwrap();
            assert_eq!(actual.as_str(), expected)
        }
    }

}
