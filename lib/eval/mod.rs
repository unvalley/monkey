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
            ast::Expression::Prefix { operator, right } => {
                let right = self.eval_expression(right)?; 
                self.eval_prefix_expression(operator, right)
            }
            _ => Err(MonkeyError::Unknown),
        }
    }

    fn eval_prefix_expression(&mut self, operator: &ast::Prefix, right: Object) -> Result<Object, MonkeyError> {
        match operator {
            ast::Prefix::Bang => match right {
                Object::Bool(value) => Ok(Object::Bool(!value)),
                Object::Null => Ok(Object::Bool(true)),
                _ => Ok(Object::Bool(false))
            },
            ast::Prefix::Minus => match right {
                Object::Integer(int) => Ok(Object::Integer(-int)),
                _ => Ok(Object::Null)
            },
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

    #[test]
    fn test_bang_operator() {
        let tests = [
            ("!true", Object::Bool(false)),
            ("!false", Object::Bool(true)),
            ("!5", Object::Bool(false)),
            ("!!true", Object::Bool(true)),
            ("!!false", Object::Bool(false)),
            ("!!5", Object::Bool(true)),
            ("5", Object::Integer(5)),
            ("10", Object::Integer(10)),
            ("-5", Object::Integer(-5)),
            ("-10", Object::Integer(-10)),
        ];
        for (input, expected) in tests {
            let actual = generate_evaluator(input);
            assert_eq!(actual, expected)
        }
    }
}
