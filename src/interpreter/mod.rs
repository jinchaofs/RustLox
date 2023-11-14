mod ast_printer;
mod environment;
mod error;
mod expression;
mod parser;
mod scanner;
mod statement;
mod token;
mod token_type;

use expression::{Expr, ExprVisitor};
use parser::Parser;
use scanner::Scanner;
use token::Token;

use crate::interpreter::ast_printer::AstPrinter;

use self::{
    environment::Environment,
    error::LoxError,
    statement::{Statement, StmtVisitor},
    token_type::{Literal, TokenType},
};

macro_rules! binary_num_operation {
    ( $left:expr, $operator:tt, $right:expr, $( $variant:ident ),+ ) => {
        match ($left, $right) {
            $(
                (Literal::$variant(left), Literal::$variant(right)) => {
                    Literal::$variant(left $operator right)
                },
            )+
            _ => Literal::None,
        }
    };
}

macro_rules! binary_bool_operation {
    ( $left:expr, $operator:tt, $right:expr, $( $variant:ident ),+ ) => {
        match ($left, $right) {
            $(
                (Literal::$variant(left), Literal::$variant(right)) => {
                    Literal::Bool(left $operator right)
                },
            )+
            _ => Literal::None,
        }
    };
}

macro_rules! binary_compare {
    ( $left:expr, ==, $right:expr, $( $variant:ident ),+ ) => {
        match ($left, $right) {
            $(
                (Literal::$variant(left), Literal::$variant(right)) => left == right,
            )+
            _ => false,
        }
    };

    ( $left:expr, !=, $right:expr, $( $variant:ident ),+ ) => {
        !binary_compare!($left, ==, $right, $( $variant ),+)
    };
}
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(None),
        }
    }
    pub fn run(&self, source: String) -> Result<(), LoxError> {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan()?;

        let parser = Parser::new(tokens);
        let statements = parser.parse()?;
        self.interpret(&statements);

        Ok(())
    }

    pub fn interpret(&self, statements: &Vec<Statement>) {
        for stmt in statements {
            self.execute(stmt);
        }
    }
    fn execute(&self, stmt: &Statement) {
        stmt.accept(self);
    }
    fn evaluate(&self, expr: &Box<Expr>) -> Literal {
        expr.accept(self)
    }
}

impl ExprVisitor for Interpreter {
    type Res = Literal;
    fn visit_binary(&self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::Res {
        let left = self.evaluate(left);
        let right = self.evaluate(right);

        match operator.ttype {
            TokenType::Minus => binary_num_operation!(left, -, right, Float, Integer),
            TokenType::Slash => binary_num_operation!(left, /, right, Float, Integer),
            TokenType::Star => binary_num_operation!(left, *, right, Float, Integer),
            TokenType::Plus => match (left, right) {
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left + right),
                (Literal::Integer(left), Literal::Integer(right)) => Literal::Integer(left + right),
                (Literal::String(left), Literal::String(right)) => Literal::String(left + &right),
                _ => Literal::None,
            },
            TokenType::Greater => binary_bool_operation!(left, >, right, Float, Integer),
            TokenType::GreaterEqual => binary_bool_operation!(left, >=, right, Float, Integer),
            TokenType::Less => binary_bool_operation!(left, <, right, Float, Integer),
            TokenType::LessEqual => binary_bool_operation!(left, <=, right, Float, Integer),
            TokenType::BangEqual => {
                Literal::Bool(binary_compare!(left, !=, right, Float, Integer, String, Bool))
            }
            TokenType::EqualEqual => {
                Literal::Bool(binary_compare!(left, ==, right, Float, Integer, String, Bool))
            }
            _ => Literal::None,
        }
    }

    fn visit_grouping(&self, expr: &Box<Expr>) -> Self::Res {
        self.evaluate(expr)
    }

    fn visit_literal(&self, literal: &Literal) -> Self::Res {
        literal.clone()
    }

    fn visit_unary(&self, operator: &Token, expr: &Box<Expr>) -> Self::Res {
        let right = self.evaluate(expr);
        match operator.ttype {
            TokenType::Bang => match right {
                Literal::Bool(val) => Literal::Bool(!val),
                Literal::Float(_) => Literal::Bool(false),
                Literal::Integer(val) => Literal::Bool(if val == 0 { true } else { false }),
                Literal::String(val) => Literal::Bool(if val.is_empty() { true } else { false }),
                _ => Literal::Bool(false),
            },
            TokenType::Minus => match right {
                Literal::Float(val) => Literal::Float(-val),
                Literal::Integer(val) => Literal::Integer(-val),
                _ => Literal::None,
            },
            _ => Literal::None,
        }
    }
    fn visit_variable(&self, name: &Token) -> Self::Res {
        self.environment.get(name).unwrap()
    }
}

impl StmtVisitor for Interpreter {
    type Res = ();
    fn visit_expression_stmt(&self, expr: &Box<Expr>) -> Self::Res {
        self.evaluate(expr);
    }
    fn visit_print_stmt(&self, expr: &Box<Expr>) -> Self::Res {
        let value = self.evaluate(expr);
        println!("{:?}", value);
    }

    fn visit_var_stmt(&self, name: &Token, initializer: &Option<Expr>) -> Self::Res {
        let mut value = Literal::None;
        if let Some(initial) = initializer {
            value = self.evaluate(&Box::new(initial.clone()));
        }
        self.environment.define(name.lexeme.clone(), value);
    }
}
