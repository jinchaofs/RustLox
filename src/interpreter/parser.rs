use std::cell::{Cell, RefCell};

use super::{
    expression::Expr,
    token::Token,
    token_type::{Literal, TokenType},
};

#[derive(Debug)]
pub struct Parser {
    tokens: RefCell<Vec<Token>>,
    current: Cell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: RefCell::new(tokens),
            current: Cell::new(0),
        }
    }

    pub fn parse(&self) -> Expr {
        self.expression()
    }

    pub fn expression(&self) -> Expr {
        self.equality()
    }

    pub fn equality(&self) -> Expr {
        let mut expr = self.comparison();

        while self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();

            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right))
        }
        expr
    }

    fn comparison(&self) -> Expr {
        let mut expr = self.term();

        while self.match_types(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn term(&self) -> Expr {
        let mut expr = self.factor();

        while self.match_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right))
        }
        expr
    }

    fn factor(&self) -> Expr {
        let mut expr = self.unary();

        while self.match_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn unary(&self) -> Expr {
        if self.match_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(operator, Box::new(right));
        }
        self.primary()
    }

    fn primary(&self) -> Expr {
        if self.match_type(TokenType::False) {
            return Expr::Literal(Literal::Bool(false));
        }
        if self.match_type(TokenType::True) {
            return Expr::Literal(Literal::Bool(true));
        }
        if self.match_type(TokenType::None) {
            return Expr::Literal(Literal::None);
        }

        if self.match_types(vec![TokenType::Number, TokenType::String]) {
            return Expr::Literal(self.previous().literal);
        }

        if self.match_type(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(Box::new(expr));
        }

        panic!("Expect expression.");
    }

    fn consume(&self, ttype: TokenType, message: &str) -> Token {
        if self.check(ttype) {
            return self.advance();
        }
        panic!("{}", message);
    }
    fn match_type(&self, ttype: TokenType) -> bool {
        if self.check(ttype) {
            self.advance();
            return true;
        }

        false
    }
    fn match_types(&self, types: Vec<TokenType>) -> bool {
        for ttype in types {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().ttype == ttype
    }

    fn advance(&self) -> Token {
        if !self.is_at_end() {
            self.current.set(self.current.get() + 1);
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens
            .borrow()
            .get(self.current.get())
            .unwrap()
            .clone()
    }

    fn previous(&self) -> Token {
        self.tokens
            .borrow()
            .get(self.current.get() - 1)
            .unwrap()
            .clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }
}
