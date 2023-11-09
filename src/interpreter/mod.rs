// mod environment;
mod expression;
mod parser;
mod scanner;
mod token;
mod token_type;

use expression::{Expr, ExprVisitor};
use parser::Parser;
use scanner::Scanner;
use token_type::TokenType;

// use self::{environment::Environment, token::Token, token_type::Literal};

pub struct Interpreter {
    // environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            // environment: Environment::new(None),
        }
    }
    pub fn run(&self, source: String) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan();
        let parser = Parser::new(tokens);
        let expr_tree = parser.parse();
        println!("expr_tree: {:#?}", expr_tree);
    }
}

// impl ExprVisitor for Interpreter {
//     fn evaluate(&self, expr: Expr) -> Result<Literal, String> {
//         match expr {
//             Expr::Assign(name, expr_value) => {
//                 let value = self.evaluate(*expr_value)?;
//                 self.environment.assign(&name, value.clone())?;
//                 return Ok(value);
//             }
//             Expr::Variable(name) => return self.environment.get(&name),
//             Expr::Binary(left, operator, right) => {
//                 let left = self.evaluate(*left)?;
//                 let right = self.evaluate(*right)?;
//                 match operator.ttype {
//                     TokenType::Minus => match_number_operate(left, right, |ln, rn| ln - rn),
//                     TokenType::Slash => match_number_operate(left, right, |ln, rn| ln / rn),
//                     TokenType::Star => match_number_operate(left, right, |ln, rn| ln * rn),
//                     TokenType::Plus => match (left, right) {
//                         (Literal::Number(ln), Literal::Number(rn)) => Ok(Literal::Number(ln + rn)),
//                         (Literal::String(left_str), Literal::String(right_str)) => {
//                             Ok(Literal::String(left_str + &right_str))
//                         }
//                         _ => Err(format!("Operants must be two numbers or two strings.")),
//                     },
//                     TokenType::Greater => match_bool_operate(left, right, |ln, rn| ln > rn),
//                     TokenType::GreaterEqual => match_bool_operate(left, right, |ln, rn| ln >= rn),
//                     TokenType::Less => match_bool_operate(left, right, |ln, rn| ln < rn),
//                     TokenType::LessEqual => match_bool_operate(left, right, |ln, rn| ln <= rn),
//                     TokenType::BangEqual => match_bool_operate(left, right, |ln, rn| ln != rn),
//                     TokenType::EqualEqual => match_bool_operate(left, right, |ln, rn| ln == rn),
//                     _ => Ok(Literal::None),
//                 }
//             }
//             Expr::Grouping(expr) => self.evaluate(*expr),
//             Expr::Unary(operator, expr) => {
//                 let right = self.evaluate(*expr)?;
//                 match operator.ttype {
//                     TokenType::Minus => match right {
//                         Literal::Number(val) => Ok(Literal::Number(-val)),
//                         _ => Err(format!("Operant must be a number.")),
//                     },
//                     TokenType::Bang => match right {
//                         Literal::Number(val) => {
//                             Ok(Literal::Bool(if val == 0.0 { true } else { false }))
//                         }
//                         Literal::Bool(val) => Ok(Literal::Bool(!val)),
//                         _ => Err(format!("Operant must be a number.")),
//                     },
//                     _ => Ok(Literal::None),
//                 }
//             }
//             Expr::Literal(literal) => Ok(literal),
//             _ => Ok(Literal::None),
//         }
//     }
// }

// fn match_number_operate<T>(left: Literal, right: Literal, operate: T) -> Result<Literal, String>
// where
//     T: Fn(f64, f64) -> f64,
// {
//     match (left, right) {
//         (Literal::Number(ln), Literal::Number(rn)) => Ok(Literal::Number(operate(ln, rn))),
//         _ => Err(format!("Operands must be numbers.")),
//     }
// }

// fn match_bool_operate<T>(left: Literal, right: Literal, operate: T) -> Result<Literal, String>
// where
//     T: Fn(f64, f64) -> bool,
// {
//     match (left, right) {
//         (Literal::Number(ln), Literal::Number(rn)) => Ok(Literal::Bool(operate(ln, rn))),
//         _ => Err(format!("Operands must be numbers.")),
//     }
// }
