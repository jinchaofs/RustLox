use std::any::Any;

use super::{token::Token, token_type::Literal};

#[derive(Debug)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Variable(Token),
    None,
}

pub trait ExprVisitor {
    fn evaluate(&self, expr: Expr) -> Result<Literal, String>;
}
