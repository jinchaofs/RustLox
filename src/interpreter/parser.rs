use std::cell::{Cell, RefCell};

use super::{
    error::LoxError,
    expression::Expr,
    statement::Statement,
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

    pub fn parse(&self) -> Result<Vec<Statement>, LoxError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&self) -> Result<Statement, LoxError> {
        if self.match_type(TokenType::Var) {
            return self.var_declaration();
        }
        return self.statement();
    }
    fn var_declaration(&self) -> Result<Statement, LoxError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let mut initializer = None;
        if self.match_type(TokenType::Equal) {
            initializer = Some(self.expression()?);
        }
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        Ok(Statement::Var(name, initializer))
    }

    fn statement(&self) -> Result<Statement, LoxError> {
        if self.match_type(TokenType::Print) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn expression_statement(&self) -> Result<Statement, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Statement::Expression(Box::new(value)))
    }

    fn print_statement(&self) -> Result<Statement, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Statement::Print(Box::new(value)))
    }

    pub fn expression(&self) -> Result<Expr, LoxError> {
        self.equality()
    }

    pub fn equality(&self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.match_types(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn term(&self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;

        while self.match_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator.clone(), Box::new(right))
        }
        Ok(expr)
    }

    fn factor(&self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;

        while self.match_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<Expr, LoxError> {
        if self.match_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&self) -> Result<Expr, LoxError> {
        if self.match_type(TokenType::False) {
            return Ok(Expr::Literal(Literal::Bool(false)));
        }
        if self.match_type(TokenType::True) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        }
        if self.match_type(TokenType::None) {
            return Ok(Expr::Literal(Literal::None));
        }

        if self.match_types(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous().literal));
        }

        if self.match_type(TokenType::Identifier) {
            return Ok(Expr::Variable(self.previous()));
        }

        if self.match_type(TokenType::LeftParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        Err(LoxError::new(
            self.peek().line,
            Some(self.peek().lexeme),
            "Expect expression.",
        ))
    }

    fn consume(&self, ttype: TokenType, message: &str) -> Result<Token, LoxError> {
        if self.check(ttype) {
            return Ok(self.advance());
        }
        Err(LoxError::new(
            self.peek().line,
            Some(self.peek().lexeme),
            message,
        ))
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
        self.tokens.borrow()[self.current.get()].clone()
    }

    fn previous(&self) -> Token {
        self.tokens.borrow()[self.current.get() - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }
}
