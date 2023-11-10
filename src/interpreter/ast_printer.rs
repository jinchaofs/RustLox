use super::{
    expression::{Expr, ExprVisitor},
    token::Token,
    token_type::Literal,
};

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(&self, expr: &Expr) {
        println!("{}", expr.accept(self));
    }
    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> String {
        let exprs_str: Vec<String> = exprs.iter().map(|expr| expr.accept(self)).collect();
        format!("({} {})", name, exprs_str.join(" "))
    }
}

impl ExprVisitor for AstPrinter {
    type Res = String;
    fn visit_binary(&self, left: &Box<Expr>, operator: &Token, right: &Box<Expr>) -> Self::Res {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_grouping(&self, expr: &Box<Expr>) -> Self::Res {
        self.parenthesize("grouping".to_string(), vec![expr])
    }

    fn visit_literal(&self, literal: &Literal) -> Self::Res {
        match literal {
            Literal::Bool(b) => b.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::Integer(i) => i.to_string(),
            Literal::None => "none".to_string(),
            Literal::String(string) => string.clone(),
        }
    }

    fn visit_unary(&self, operator: &Token, expr: &Box<Expr>) -> Self::Res {
        self.parenthesize(operator.lexeme.clone(), vec![expr])
    }
}
