use std::{cell::RefCell, rc::Rc};

use crate::eval::object::{Object, ObjectType};
use crate::{
    error::MonkeyError,
    eval::environment::Environment,
    parser::ast::{self, Statement},
};

pub mod environment;
pub mod object;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Evaluator {
    // Why do we need Rc & Refcell ?
    env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn from_env(env: Environment) -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(env.to_owned())),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<Object> {
        self.env.borrow_mut().get(key)
    }

    pub fn set(&mut self, key: String, value: Object) -> Option<Object> {
        self.env.borrow_mut().set(key, value)
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
            ast::Statement::Return(expr) => {
                let obj = self.eval_expression(expr)?;
                Ok(Object::Return(Box::new(obj)))
            }
            ast::Statement::Let { identifier, value } => {
                if let ast::Expression::Identifier(ident) = identifier {
                    let val = self.eval_expression(value)?;
                    self.set(ident.to_owned(), val);
                    Ok(Object::Null)
                } else {
                    panic!("panic at let statement evaluation")
                }
            }
            _ => Err(MonkeyError::Unknown),
        }
    }

    fn eval_expression(&mut self, expr: &ast::Expression) -> Result<Object, MonkeyError> {
        match expr {
            ast::Expression::Integer(int) => Ok(Object::Integer(*int)),
            ast::Expression::String(str) => Ok(Object::String(str.to_owned())),
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
            ast::Expression::Identifier(ident) => match self.get(ident) {
                Some(val) => Ok(val),
                None => Err(MonkeyError::IdentifierNotFound),
            },
            ast::Expression::Function { parameters, body } => Ok(Object::Function {
                parameters: parameters.clone(),
                body: *body.clone(),
                env: Environment::new_enclosed(Rc::clone(&self.env)),
            }),
            ast::Expression::Call {
                function,
                arguments,
            } => {
                let args = self.eval_expressions(arguments)?;
                let function = self.eval_expression(function)?;
                // We have to evaluate inside of function.

                self.apply_function(function, args)
            }
            _ => Err(MonkeyError::Unknown),
        }
    }

    fn eval_expressions(&mut self, exprs: &[ast::Expression]) -> Result<Vec<Object>, MonkeyError> {
        let mut result = vec![];
        for expr in exprs.iter() {
            result.push(self.eval_expression(expr)?)
        }
        Ok(result)
    }

    fn apply_function(
        &mut self,
        function: Object,
        args: Vec<Object>,
    ) -> Result<Object, MonkeyError> {
        if let Object::Function {
            parameters,
            body,
            env,
        } = function
        {
            if parameters.len() != args.len() {
                return Err(MonkeyError::IncorrectNumberOfArguments {
                    expected: args.len(),
                    actual: parameters.len(),
                });
            }
            let mut evaluator = Evaluator::from_env(env);
            for (ident, arg) in parameters.iter().zip(args.iter()) {
                if let ast::Expression::Identifier(ident) = ident {
                    evaluator.set(ident.to_owned(), arg.clone());
                }
            }
            match evaluator.eval_statement(&body)? {
                Object::Return(obj) => Ok(*obj),
                obj => Ok(obj),
            }
        } else {
            Err(MonkeyError::Unknown)
        }
    }

    fn eval_block_statement(&mut self, stmts: &[Statement]) -> Result<Object, MonkeyError> {
        let mut result = Object::Null;
        for stmt in stmts.iter() {
            result = self.eval_statement(stmt)?;

            if let Object::Return(_) = result {
                return Ok(result);
            }
        }
        // for block statement
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
                    left: ObjectType::Bool,
                    operator: *operator,
                    right: ObjectType::Bool,
                }),
            },
            (Object::Integer(_), Object::Bool(_)) => Err(MonkeyError::TypeMismatch {
                operator: *operator,
                left: ObjectType::Integer,
                right: ObjectType::Bool,
            }),
            (Object::Bool(_), Object::Integer(_)) => Err(MonkeyError::TypeMismatch {
                operator: *operator,
                left: ObjectType::Bool,
                right: ObjectType::Integer,
            }),
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
        error::MonkeyError,
        eval::{environment::Environment, Evaluator, Object, ObjectType},
        lexer::Lexer,
        parser::{
            ast::{self, Expression, Program},
            Parser,
        },
    };

    fn generate_program(input: &str) -> Program {
        let l = Lexer::new(input.to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        program
    }

    fn evaluate_program(input: &str) -> Object {
        let program = generate_program(input);
        let mut eval = Evaluator::new();
        eval.evaluate(&program).unwrap()
    }

    fn evaluate_error_program(input: &str) -> MonkeyError {
        let program = generate_program(input);
        let mut eval = Evaluator::new();
        match eval.evaluate(&program) {
            Ok(_) => panic!("Expected error."),
            Err(err) => err,
        }
    }

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
            let actual = evaluate_program(input);
            assert_eq!(actual, expected)
        }
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
            (r#""Hello World!""#, Object::String("Hello World!".to_string()))
        ];
        for (input, expected) in tests {
            let actual = evaluate_program(input);
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
            let actual = evaluate_program(input);
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn test_let_statements() {
        let tests = [
            ("let a = 5; a;", Object::Integer(5)),
            ("let a = 5 * 5; a;", Object::Integer(25)),
            ("let a = 5; let b = a; b;", Object::Integer(5)),
            (
                "let a = 5; let b = a; let c = a + b + 5; c;",
                Object::Integer(15),
            ),
        ];
        for (input, expected) in tests {
            let actual = evaluate_program(input);
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = [
            ("return 10;", Object::Integer(10)),
            ("return 10; 9;", Object::Integer(10)),
            ("return 2 * 5; 9;", Object::Integer(10)),
            ("9; return 2 * 5; 9;", Object::Integer(10)),
            (
                "if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }
            ",
                Object::Integer(10),
            ),
        ];

        for (input, expected) in tests {
            let actual = evaluate_program(input);
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = [
            (
                "5 + true;",
                MonkeyError::TypeMismatch {
                    operator: ast::Infix::Plus,
                    left: ObjectType::Integer,
                    right: ObjectType::Bool,
                },
            ),
            (
                "5 + true; 5",
                MonkeyError::TypeMismatch {
                    operator: ast::Infix::Plus,
                    left: ObjectType::Integer,
                    right: ObjectType::Bool,
                },
            ),
            ("foobar", MonkeyError::IdentifierNotFound),
        ];
        for (input, expected) in tests {
            let actual = evaluate_error_program(input);
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn test_function_object() {
        let tests = [("fn(x) { x + 2;};")];
        for input in tests {
            let actual = evaluate_program(input);
            if let Object::Function {
                parameters,
                body,
                env,
            } = actual
            {
                assert_eq!(parameters.len(), 1);
                assert_eq!(format!("{}", parameters[0]), "x".to_string());
                // assert_eq!(format!("{}", body), "(x + 2)".to_string())
            }
        }
    }

    #[test]
    fn test_function_application() {
        let tests = [
            (
                "let identity = fn(x) {x;}; identity(5);",
                Object::Integer(5),
            ),
            (
                "let identity = fn(x) { return x; }; identity(5);",
                Object::Integer(5),
            ),
            (
                "let double = fn(x) {x * 2}; double(5);",
                Object::Integer(10),
            ),
            (
                "let double = fn(x) {x * 2}; double(5);",
                Object::Integer(10),
            ),
            ("let add = fn(x,y) {x + y}; add(5, 5);", Object::Integer(10)),
            (
                "let add = fn(x,y) {x + y}; add(5+5, add(5,5));",
                Object::Integer(20),
            ),
            ("fn(x) {x;}(5)", Object::Integer(5)),
        ];

        for (input, expected) in tests {
            let actual = evaluate_program(input);
            assert_eq!(actual, expected);
        }
    }
}
