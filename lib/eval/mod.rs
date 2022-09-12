use std::fmt;

use crate::{error::MonkeyError, parser::ast};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn evaluate(&mut self, program: &ast::Program) -> Result<Object, MonkeyError> {
        let mut result = Object::Null;
        for stmt in &program.statements {
            result = self.eval_statement(stmt)?;
            if let Object::Return(return_value) = result {
                return Ok(*return_value);
            }
        }
        Ok(result)
    }

    fn eval_statement(&mut self, stmt: &ast::Statement) -> Result<Object, MonkeyError> {
        match stmt {
            ast::Statement::Expression(expr) => self.eval_expression(expr),
            _ => Err(MonkeyError::Unknown),
        }
    }

    fn eval_expression(&mut self, expr: &ast::Expression) -> Result<Object, MonkeyError> {
        match expr {
            ast::Expression::Integer(int) => Ok(Object::Integer(*int)),
            ast::Expression::Boolean(bool) => Ok(Object::Bool(*bool)),
            _ => Err(MonkeyError::Unknown),
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        eval::{Evaluator, Object},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_eval_expressions() {
        let tests = [
            ("5", Object::Integer(5)),
            ("10", Object::Integer(10)),
            ("true", Object::Bool(true)),
            ("false", Object::Bool(false)),
        ];

        for (input, expected) in tests {
            let actual = generate_evaluator(input);
            assert_eq!(actual, expected)
        }
    }

    fn generate_evaluator(input: &str) -> Object {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        let mut eval = Evaluator::new();
        eval.evaluate(&program).unwrap()
    }
}
