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
            return Err(MonkeyError::UnexpectedToken {
                expected: token::Token::Identifier("".to_string()),
                actual: self.current_token.clone(),
            });
        };
        self.expect_peek(token::Token::Assign)?;
        self.next_token();
        let value = self.parse_expression(ast::Precedence::Lowest)?;

        if !self.is_current_token(token::Token::SemiColon) {
            self.next_token()
        }

        Ok(ast::Statement::Let { identifier, value })
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
    fn parse_expression(
        &mut self,
        precedence: ast::Precedence,
    ) -> Result<ast::Expression, MonkeyError> {
        let mut left_exp = match &self.current_token {
            token::Token::Identifier(ident) => ast::Expression::Identifier(ident.to_owned()),
            token::Token::StringLiteral(str) => ast::Expression::String(str.to_owned()),
            token::Token::IntLiteral(int) => ast::Expression::Integer(*int),
            token::Token::True => ast::Expression::Boolean(true),
            token::Token::False => ast::Expression::Boolean(false),
            token::Token::Bang => self.parse_prefix_expression()?,
            token::Token::Minus => self.parse_prefix_expression()?,
            token::Token::LParen => self.parse_grouped_expression()?,
            token::Token::If => self.parse_if_expression()?,
            token::Token::Function => self.parse_function_expression()?,
            token => return Err(MonkeyError::InvalidToken(token.clone())),
        };

        // 中間演算子の処理
        while !self.is_peek_token(token::Token::SemiColon) && precedence < self.peek_precedence() {
            // tokenが見つかったら，対象の中間演算子がcurrent_tokenに来るようにnext_token()を実行
            match self.peek_token {
                token::Token::Plus => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                token::Token::Minus => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                token::Token::Slash => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                token::Token::Asterisk => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                token::Token::Eq => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                token::Token::NotEq => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                token::Token::LT => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                token::Token::GT => {
                    self.next_token();
                    left_exp = self.parse_infix_expression(left_exp)?;
                }
                token::Token::LParen => {
                    self.next_token();
                    left_exp = self.parse_call_expression(left_exp)?;
                }
                // TODO: LParen
                _ => return Ok(left_exp),
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
            _ => return Err(MonkeyError::InvalidToken(self.current_token.clone())),
        };
        self.next_token();
        // 優先順位としてPrefix渡す．なぜならこの関数が前置演算子式をparseしている最中だから
        let right = self.parse_expression(ast::Precedence::Prefix)?;
        Ok(ast::Expression::Prefix {
            operator: op,
            right: Box::new(right),
        })
    }

    fn parse_infix_expression(
        &mut self,
        left_expression: ast::Expression,
    ) -> Result<ast::Expression, MonkeyError> {
        let operator = match self.current_token {
            token::Token::Plus => ast::Infix::Plus,
            token::Token::Minus => ast::Infix::Minus,
            token::Token::Asterisk => ast::Infix::Asterisk,
            token::Token::Slash => ast::Infix::Slash,
            token::Token::Eq => ast::Infix::Eq,
            token::Token::NotEq => ast::Infix::NotEq,
            token::Token::LT => ast::Infix::LT,
            token::Token::GT => ast::Infix::GT,
            _ => return Err(MonkeyError::InvalidToken(self.current_token.clone())),
        };
        let precedence = self.current_precedence();
        self.next_token();
        let right_expression = self.parse_expression(precedence)?;
        Ok(ast::Expression::Infix {
            operator,
            left: Box::new(left_expression),
            right: Box::new(right_expression),
        })
    }

    fn parse_grouped_expression(&mut self) -> Result<ast::Expression, MonkeyError> {
        self.next_token();
        let expr = self.parse_expression(ast::Precedence::Lowest)?;
        self.expect_peek(token::Token::RParen)?;
        Ok(expr)
    }

    fn parse_if_expression(&mut self) -> Result<ast::Expression, MonkeyError> {
        self.expect_peek(token::Token::LParen)?;
        let condition = Box::new(self.parse_expression(ast::Precedence::Lowest)?);

        if !self.is_current_token(token::Token::RParen) {
            return Err(MonkeyError::UnexpectedToken {
                expected: token::Token::RParen,
                actual: self.current_token.clone(),
            });
        }
        self.expect_peek(token::Token::LBrace)?;
        let consequence = Box::new(self.parse_block_statement()?);
        let alternative = if self.is_peek_token(token::Token::Else) {
            self.next_token();
            self.expect_peek(token::Token::LBrace)?;
            let stmt = self.parse_block_statement()?;
            Some(Box::new(stmt))
        } else {
            None
        };

        Ok(ast::Expression::If {
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_function_expression(&mut self) -> Result<ast::Expression, MonkeyError> {
        self.expect_peek(token::Token::LParen)?;
        let parameters = self.parse_function_parameters()?;
        // ) → {
        self.expect_peek(token::Token::LBrace)?;
        let body = self.parse_block_statement()?;
        Ok(ast::Expression::Function {
            parameters,
            body: Box::new(body),
        })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<ast::Expression>, MonkeyError> {
        let mut identifiers = vec![];

        let no_arguments = self.is_peek_token(token::Token::RParen);
        if no_arguments {
            self.next_token();
            return Ok(identifiers);
        }

        // handle argments

        self.next_token();
        if let token::Token::Identifier(ident) = &self.current_token {
            identifiers.push(ast::Expression::Identifier(ident.to_owned()));
        } else {
            return Err(MonkeyError::InvalidToken(self.current_token.clone()));
        }

        while self.is_peek_token(token::Token::Comma) {
            self.next_token();
            self.next_token();
            if let token::Token::Identifier(ident) = &self.current_token {
                identifiers.push(ast::Expression::Identifier(ident.to_owned()));
            } else {
                return Err(MonkeyError::InvalidToken(self.current_token.clone()));
            }
        }
        self.expect_peek(token::Token::RParen)?;
        Ok(identifiers)
    }

    fn parse_call_expression(
        &mut self,
        function: ast::Expression,
    ) -> Result<ast::Expression, MonkeyError> {
        let arguments = self.parse_call_arguments()?;
        let expr = ast::Expression::Call {
            function: Box::new(function),
            arguments,
        };
        Ok(expr)
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<ast::Expression>, MonkeyError> {
        let mut arguments: Vec<ast::Expression> = Vec::new();
        if self.is_peek_token(token::Token::RParen) {
            self.next_token();
            return Ok(arguments);
        };
        self.next_token();
        arguments.push(self.parse_expression(ast::Precedence::Lowest)?);

        while self.is_peek_token(token::Token::Comma) {
            self.next_token();
            self.next_token();
            arguments.push(self.parse_expression(ast::Precedence::Lowest)?);
        }

        self.expect_peek(token::Token::RParen)?;
        Ok(arguments)
    }

    fn parse_block_statement(&mut self) -> Result<ast::Statement, MonkeyError> {
        self.next_token();
        let mut statements: Vec<ast::Statement> = vec![];
        while !self.is_current_token(token::Token::RBrace)
            && !self.is_current_token(token::Token::EOF)
        {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.next_token();
        }
        Ok(ast::Statement::Block(statements))
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
            Err(MonkeyError::UnexpectedToken {
                expected: token,
                actual: self.peek_token.clone(),
            })
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
        let tests = vec![
            ("let x = 5;", "x", ast::Expression::Integer(5)),
            ("let y = true;", "y", ast::Expression::Boolean(true)),
            (
                "let foobar = y;",
                "foobar",
                ast::Expression::Identifier("y".to_string()),
            ),
        ];

        for test in tests.iter() {
            // doesn't mock lexer for readability
            let l = Lexer::new(test.0.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            assert_eq!(program.statements.len(), 1);
            let stmt = &program.statements[0];
            if let ast::Statement::Let { identifier, .. } = stmt {
                if let ast::Expression::Identifier(identifier) = identifier {
                    assert_eq!(identifier, test.1)
                } else {
                    panic!("expected ast::Statement::Let, but got {:?}", stmt);
                }
            }
            if let ast::Statement::Let {
                identifier: _,
                value,
            } = stmt
            {
                match value {
                    ast::Expression::Integer(v) => assert_eq!(ast::Expression::Integer(*v), test.2),
                    ast::Expression::Boolean(v) => assert_eq!(ast::Expression::Boolean(*v), test.2),
                    ast::Expression::Identifier(v) => {
                        assert_eq!(ast::Expression::Identifier(v.to_string()), test.2)
                    }
                    err_expr => panic!(
                        "expected ast::Expression::(Integer|Boolean|Identifier), but got {:?}",
                        err_expr
                    ),
                }
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
        struct PrefixExpressionTest {
            input: String,
            operator: ast::Prefix,
            right: ast::Expression,
        }
        let prefix_tests = vec![
            PrefixExpressionTest {
                input: "!5;".to_string(),
                operator: ast::Prefix::Bang,
                right: ast::Expression::Integer(5),
            },
            PrefixExpressionTest {
                input: "-15;".to_string(),
                operator: ast::Prefix::Minus,
                right: ast::Expression::Integer(15),
            },
        ];
        for test in prefix_tests.iter() {
            let l = Lexer::new(test.input.clone());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            assert_eq!(program.statements.len(), 1);
            // TODO:
            // let expr = match &program.statements[0] {
            //     ast::Statement::Expression(expr) => expr,
            //     _ => panic!("program.statements should have Expression, but got {:?}", program.statements[0]),
            // };
        }
    }

    #[test]
    fn test_parse_infix_expressions() {
        struct InfixExpressionTest {
            input: String,
            left: ast::Expression,
            operator: ast::Infix,
            right: ast::Expression,
        }
        let infix_tests = vec![
            InfixExpressionTest {
                input: "5+5;".to_string(),
                left: ast::Expression::Integer(5),
                operator: ast::Infix::Plus,
                right: ast::Expression::Integer(5),
            },
            InfixExpressionTest {
                input: "5-5;".to_string(),
                left: ast::Expression::Integer(5),
                operator: ast::Infix::Minus,
                right: ast::Expression::Integer(5),
            },
            InfixExpressionTest {
                input: "5*5;".to_string(),
                left: ast::Expression::Integer(5),
                operator: ast::Infix::Asterisk,
                right: ast::Expression::Integer(5),
            },
            InfixExpressionTest {
                input: "5/5;".to_string(),
                left: ast::Expression::Integer(5),
                operator: ast::Infix::Slash,
                right: ast::Expression::Integer(5),
            },
            InfixExpressionTest {
                input: "5>5;".to_string(),
                left: ast::Expression::Integer(5),
                operator: ast::Infix::GT,
                right: ast::Expression::Integer(5),
            },
            InfixExpressionTest {
                input: "5<5;".to_string(),
                left: ast::Expression::Integer(5),
                operator: ast::Infix::LT,
                right: ast::Expression::Integer(5),
            },
            InfixExpressionTest {
                input: "5==5;".to_string(),
                left: ast::Expression::Integer(5),
                operator: ast::Infix::Eq,
                right: ast::Expression::Integer(5),
            },
            InfixExpressionTest {
                input: "5!=5;".to_string(),
                left: ast::Expression::Integer(5),
                operator: ast::Infix::NotEq,
                right: ast::Expression::Integer(5),
            },
            InfixExpressionTest {
                input: "true==true;".to_string(),
                left: ast::Expression::Boolean(true),
                operator: ast::Infix::Eq,
                right: ast::Expression::Boolean(true),
            },
            InfixExpressionTest {
                input: "true!=false;".to_string(),
                left: ast::Expression::Boolean(false),
                operator: ast::Infix::NotEq,
                right: ast::Expression::Boolean(false),
            },
            InfixExpressionTest {
                input: "false==false;".to_string(),
                left: ast::Expression::Boolean(false),
                operator: ast::Infix::Eq,
                right: ast::Expression::Boolean(false),
            },
        ];
        for test in infix_tests.iter() {
            let l = Lexer::new(test.input.clone());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            assert_eq!(program.statements.len(), 1);
            // TODO: more strict test
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            ("-a*b", "((-a)*b)"),
            ("5+5*10", "(5+(5*10))"),
            ("1+(2+3)+4", "((1+(2+3))+4)"),
            ("(5+5)*2", "((5+5)*2)"),
            ("2/(5+5)", "(2/(5+5))"),
            ("-(5+5)", "(-(5+5))"),
            ("!(true==true)", "(!(true==true))"),
            ("a+add(b*c)+d", "((a+add((b*c)))+d)"),
            (
                "add(a,b,1,2*3,4+5,add(6,7*8))",
                "add(a,b,1,(2*3),(4+5),add(6,(7*8)))",
            ),
            ("add(a+b+c*d/f+g)", "add((((a+b)+((c*d)/f))+g))"),
        ];
        for (input, expected) in tests {
            let l = Lexer::new(input.to_string());
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            assert_eq!(program.statements.len(), 1);
            let stmt = &program.statements[0];
            if let ast::Statement::Expression(expr) = stmt {
                assert_eq!(&format!("{}", &expr), expected);
            } else {
                panic!("Incorrect statement")
            }
        }
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x } else { y }";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        if let ast::Statement::Expression(expr) = stmt {
            if let ast::Expression::If {
                condition,
                consequence,
                alternative,
            } = expr
            {
                assert_eq!(format!("{}", condition), "(x<y)");
                // assert_eq!(format!("{}", consequence), "x");
                // if let Some(alternative) = alternative {
                //     assert_eq!(format!("{}", alternative), "y")
                // }
            } else {
                panic!("Incorrect expression");
            }
        } else {
            panic!("Incorrect statement");
        }
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y }";
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        if let ast::Statement::Expression(expr) = stmt {
            if let ast::Expression::Function {
                parameters,
                body: _,
            } = expr
            {
                assert_eq!(parameters.len(), 2);
                assert_eq!(format!("{}", parameters[0]), "x");
                assert_eq!(format!("{}", parameters[1]), "y");
                // TODO: body test
            } else {
                panic!("Incorrect expression")
            }
        } else {
            panic!("Incorrect statement")
        }
    }

    #[test]
    fn test_function_parameter_parsing() {
        struct FunctionalParameterTest {
            input: String,
            expected_params: Vec<String>,
        }
        let tests = vec![
            FunctionalParameterTest {
                input: "fn() {}".to_string(),
                expected_params: vec![],
            },
            FunctionalParameterTest {
                input: "fn(x) {}".to_string(),
                expected_params: vec!["x".to_string()],
            },
            FunctionalParameterTest {
                input: "fn(x,y,z) {}".to_string(),
                expected_params: vec!["x".to_string(), "y".to_string(), "z".to_string()],
            },
        ];
        for test in tests {
            let l = Lexer::new(test.input);
            let mut p = Parser::new(l);
            let program = p.parse_program().unwrap();
            let stmt = &program.statements[0];
            if let ast::Statement::Expression(expr) = stmt {
                match expr {
                    ast::Expression::Function { parameters, body } => {
                        assert_eq!(parameters.len(), test.expected_params.len());
                        for (idx, expected) in test.expected_params.iter().enumerate() {
                            assert_eq!(format!("{}", parameters[idx]), *expected)
                        }
                    }
                    _ => panic!("Incorrect expression."),
                }
            } else {
                panic!("Incorrect statements.")
            }
        }
    }

    #[test]
    fn test_call_expression_parameter_parsing() {
        let input = "add(1, 2 * 3, 4 + 5)".to_string();
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        if let ast::Statement::Expression(expr) = stmt {
            match expr {
                ast::Expression::Call {
                    function,
                    arguments,
                } => {
                    assert_eq!(format!("{}", function), "add");
                    assert_eq!(arguments.len(), 3);
                }
                _ => panic!("Incorrect expressions"),
            }
        } else {
            panic!("Incorrect statements")
        }
    }

    #[test]
    fn test_string_literal_expression() {
        let input = r#""hello world""#.to_string();
        let l = Lexer::new(input.clone());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        assert_eq!(program.statements.len(), 1);
        let stmt = &program.statements[0];
        if let ast::Statement::Expression(expr) = stmt {
            match expr {
                ast::Expression::String(str) => {
                    assert_eq!("hello world", str)
                },
                _ => panic!("Incorrect expressions")
            }
        } else {
            panic!("Incorrect statements")
        }
    }
}
