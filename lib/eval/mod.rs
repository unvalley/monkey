use std::fmt;

use crate::{
    error::MonkeyError,
    parser::ast::{self, Expression, Statement},
};

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

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Object::Null => false,
            Object::Bool(value) => *value,
            _ => true,
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
            ast::Statement::Block(stmts) => self.eval_block_statement(stmts),
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
            ast::Expression::Infix {
                operator,
                left,
                right,
            } => {
                let right = self.eval_expression(right)?;
                let left = self.eval_expression(left)?;
                self.eval_infix_expression(operator, left, right)
            }
            ast::Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                if self.eval_expression(condition)?.is_truthy() {
                    self.eval_statement(consequence)
                } else {
                    match alternative {
                        Some(alt) => self.eval_statement(alt),
                        None => Ok(Object::Null),
                    }
                }
            }
            _ => Err(MonkeyError::Unknown),
        }
    }

    fn eval_block_statement(&mut self, stmts: &Vec<Statement>) -> Result<Object, MonkeyError> {
        let mut result = Object::Null;
        for stmt in stmts.iter() {
            result = self.eval_statement(stmt)?;
            if let Object::Return(return_value) = result {
                return Ok(*return_value);
            }
        }
        Ok(result)
    }

    fn eval_infix_expression(
        &mut self,
        operator: &ast::Infix,
        left: Object,
        right: Object,
    ) -> Result<Object, MonkeyError> {
        match (left, right) {
            (Object::Integer(left), Object::Integer(right)) => match operator {
                ast::Infix::Eq => Ok(Object::Bool(left == right)),
                ast::Infix::NotEq => Ok(Object::Bool(left != right)),
                ast::Infix::LT => Ok(Object::Bool(left < right)),
                ast::Infix::GT => Ok(Object::Bool(left > right)),
                ast::Infix::Plus => Ok(Object::Integer(left + right)),
                ast::Infix::Minus => Ok(Object::Integer(left - right)),
                ast::Infix::Slash => Ok(Object::Integer(left / right)),
                ast::Infix::Asterisk => Ok(Object::Integer(left * right)),
            },
            (Object::Bool(left), Object::Bool(right)) => match operator {
                ast::Infix::Eq => Ok(Object::Bool(left == right)),
                ast::Infix::NotEq => Ok(Object::Bool(left != right)),
                operator => Err(MonkeyError::UnknownOperator {
                    expected: ObjectType::Bool,
                    actual: *operator,
                }),
            },
            _ => Ok(Object::Null),
        }
    }

    fn eval_prefix_expression(
        &mut self,
        operator: &ast::Prefix,
        right: Object,
    ) -> Result<Object, MonkeyError> {
        match operator {
            ast::Prefix::Bang => match right {
                Object::Bool(value) => Ok(Object::Bool(!value)),
                Object::Null => Ok(Object::Bool(true)),
                _ => Ok(Object::Bool(false)),
            },
            ast::Prefix::Minus => match right {
                Object::Integer(int) => Ok(Object::Integer(-int)),
                _ => Ok(Object::Null),
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
            ("-5", Object::Integer(-5)),
            ("-10", Object::Integer(-10)),
            ("5 + 5 + 5 + 5 - 10", Object::Integer(10)),
            ("2 * 2 * 2 * 2 * 2", Object::Integer(32)),
            ("-50 + 100 + -50", Object::Integer(0)),
            ("5 * 2 + 10", Object::Integer(20)),
            ("5 + 2 * 10", Object::Integer(25)),
            ("20 + 2 * -10", Object::Integer(0)),
            ("50 / 2 * 2 + 10", Object::Integer(60)),
            ("2 * (5 + 10)", Object::Integer(30)),
            ("3 * 3 * 3 + 10", Object::Integer(37)),
            ("3 * (3 * 3) + 10", Object::Integer(37)),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50)),
            ("true == true", Object::Bool(true)),
            ("false == false", Object::Bool(true)),
            ("true == false", Object::Bool(false)),
            ("true != false", Object::Bool(true)),
            ("false != true", Object::Bool(true)),
            ("(1 < 2) == true", Object::Bool(true)),
            ("(1 < 2) == false", Object::Bool(false)),
            ("(1 > 2) == true", Object::Bool(false)),
            ("(1 > 2) == false", Object::Bool(true)),
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
            ("1 < 2", Object::Bool(true)),
            ("1 > 2", Object::Bool(false)),
            ("1 < 1", Object::Bool(false)),
            ("1 > 1", Object::Bool(false)),
            ("1 == 1", Object::Bool(true)),
            ("1 == 2", Object::Bool(false)),
            ("1 != 2", Object::Bool(true)),
        ];
        for (input, expected) in tests {
            let actual = generate_evaluator(input);
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn test_if_else_expression() {
        let tests = [
            ("if (true) { 10 }", Object::Integer(10)),
            ("if (false) { 10 }", Object::Null),
            ("if (1) { 10 }", Object::Integer(10)),
            ("if (1 < 2) { 10 }", Object::Integer(10)),
            ("if (1 > 2) { 10 }", Object::Null),
            ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
        ];

        for (input, expected) in tests {
            let actual = generate_evaluator(input);
            assert_eq!(actual, expected)
        }
    }
}
