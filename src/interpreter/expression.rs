use super::{token::Token, token_type::Literal};

#[derive(Debug, Clone)]
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
    type Res;
    fn visit_binary(&self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::Res;
    fn visit_grouping(&self, expr: &Box<Expr>) -> Self::Res;
    fn visit_literal(&self, literal: &Literal) -> Self::Res;
    // fn evaluate_assign(&self, token: &Token, expr: &Box<Expr>) -> Self::Res;
    fn visit_unary(&self, operator: &Token, expr: &Box<Expr>) -> Self::Res;
}

impl Expr {
    pub fn accept<V: ExprVisitor>(&self, visitor: &V) -> V::Res {
        match self {
            Expr::Binary(left, operator, right) => visitor.visit_binary(left, operator, right),
            Expr::Grouping(expr) => visitor.visit_grouping(expr),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Unary(operator, expr) => visitor.visit_unary(operator, expr),
            _ => unimplemented!(),
        }
    }
}
