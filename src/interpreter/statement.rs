use super::{expression::Expr, token::Token};

#[derive(Debug)]
pub enum Statement {
    Print(Box<Expr>),
    Expression(Box<Expr>),
    Var(Token, Option<Expr>),
}

impl Statement {
    pub fn accept<V: StmtVisitor>(&self, visitor: &V) -> V::Res {
        match self {
            Statement::Print(expr) => visitor.visit_print_stmt(expr),
            Statement::Expression(expr) => visitor.visit_expression_stmt(expr),
            Statement::Var(name, expr) => visitor.visit_var_stmt(name, expr),
        }
    }
}

pub trait StmtVisitor {
    type Res;
    fn visit_print_stmt(&self, expr: &Box<Expr>) -> Self::Res;
    fn visit_expression_stmt(&self, expr: &Box<Expr>) -> Self::Res;
    fn visit_var_stmt(&self, name: &Token, initializer: &Option<Expr>) -> Self::Res;
}
