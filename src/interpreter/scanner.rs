use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use super::{
    error::LoxError,
    token::Token,
    token_type::{Literal, TokenType},
};

pub struct Scanner {
    source: String,
    source_chars: Vec<char>,
    tokens: RefCell<Vec<Token>>,

    start: Cell<usize>,
    current: Cell<usize>,
    line: Cell<usize>,

    keywords: RefCell<HashMap<String, TokenType>>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source.clone(),
            source_chars: source.chars().collect(),
            tokens: RefCell::new(vec![]),
            start: Cell::new(0),
            current: Cell::new(0),
            line: Cell::new(1),
            keywords: RefCell::new(Self::init_keywords()),
        }
    }
    fn init_keywords() -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();
        let keyword_tokens = [
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::None),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ];

        for (keyword, token_type) in keyword_tokens.iter() {
            keywords.insert(keyword.to_string(), *token_type);
        }

        keywords
    }

    pub fn scan(&self) -> Result<Vec<Token>, LoxError> {
        while !self.end_of_source() {
            self.start.set(self.current.get());
            self.scan_token()?;
        }

        self.tokens.borrow_mut().push(Token::new(
            TokenType::Eof,
            "".into(),
            Literal::None,
            self.line.get(),
        ));

        Ok(self.tokens.borrow().to_vec())
    }

    fn scan_token(&self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.put_token(TokenType::LeftParen),
            ')' => self.put_token(TokenType::RightParen),
            '{' => self.put_token(TokenType::LeftBrace),
            '}' => self.put_token(TokenType::RightBrace),
            ',' => self.put_token(TokenType::Comma),
            '.' => self.put_token(TokenType::Dot),
            '-' => self.put_token(TokenType::Minus),
            '+' => self.put_token(TokenType::Plus),
            ';' => self.put_token(TokenType::Semicolon),
            '*' => self.put_token(TokenType::Star),
            '!' => self.put_token(if self.next_match('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }),
            '=' => self.put_token(if self.next_match('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),
            '<' => self.put_token(if self.next_match('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),
            '>' => self.put_token(if self.next_match('=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),
            '/' => {
                if self.next_match('/') {
                    while !self.end_of_line() && !self.end_of_source() {
                        self.advance();
                    }
                    Ok(())
                } else if self.next_match('*') {
                    // multi line comments
                    Ok(())
                } else {
                    self.put_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => self.line_advance(),
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    Err(LoxError::new(
                        self.line.get(),
                        None,
                        "Unexpected character.",
                    ))
                }
            }
        }
    }

    fn identifier(&self) -> Result<(), LoxError> {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start.get()..self.current.get()];
        let ttype = self.keywords.borrow().get(text).cloned();
        if let Some(ttype) = ttype {
            self.put_token(ttype)?;
            return Ok(());
        }

        self.put_token(TokenType::Identifier)
    }

    fn number(&self) -> Result<(), LoxError> {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start.get()..self.current.get()];

        let value = if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
            text.parse::<f64>()
                .map(Literal::Float)
                .map_err(|e| format!("Failed to parse the value({}) to float. {}", text, e))
        } else {
            text.parse::<i64>()
                .map(Literal::Integer)
                .map_err(|e| format!("Failed to parse the value({}) to integer. {}", text, e))
        };

        match value {
            Ok(val) => self.put_token_with_literal(TokenType::Number, val),
            Err(e) => Err(LoxError::new(self.line.get(), None, &e)),
        }
    }

    fn string(&self) -> Result<(), LoxError> {
        self.advance_with_condition(|| self.peek() != '"')?;

        if self.end_of_source() {
            return Err(LoxError::new(self.line.get(), None, "Unterminated string."));
        }

        self.advance();

        let text = &self.source[(self.start.get() + 1)..(self.current.get() - 1)];
        self.put_token_with_literal(TokenType::String, Literal::String(text.into()))
    }

    fn put_token(&self, ttype: TokenType) -> Result<(), LoxError> {
        self.put_token_with_literal(ttype, Literal::None)
    }

    fn put_token_with_literal(&self, ttype: TokenType, literal: Literal) -> Result<(), LoxError> {
        let text = &self.source[self.start.get()..self.current.get()];
        self.tokens
            .borrow_mut()
            .push(Token::new(ttype, text.into(), literal, self.line.get()));
        Ok(())
    }

    fn advance_with_condition<F>(&self, condition: F) -> Result<(), LoxError>
    where
        F: Fn() -> bool,
    {
        while condition() && !self.end_of_source() {
            if self.end_of_line() {
                self.line_advance()?;
            }
            self.advance();
        }
        Ok(())
    }

    fn advance(&self) -> char {
        let c = self.source_chars[self.current.get()];
        self.current.set(self.current.get() + 1);
        c
    }

    fn line_advance(&self) -> Result<(), LoxError> {
        self.line.set(self.line.get() + 1);
        Ok(())
    }

    fn peek(&self) -> char {
        if self.end_of_source() {
            return '\0';
        }
        self.source_chars[self.current.get()]
    }

    fn peek_next(&self) -> char {
        let next = self.current.get() + 1;
        if next >= self.source.len() {
            return '\0';
        }
        self.source_chars[next]
    }

    fn next_match(&self, expected: char) -> bool {
        if self.end_of_source() {
            return false;
        }
        if self.source_chars[self.current.get()] != expected {
            return false;
        }
        self.advance();
        return true;
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn end_of_line(&self) -> bool {
        self.peek() == '\n'
    }

    fn end_of_source(&self) -> bool {
        self.current.get() >= self.source.len()
    }
}
