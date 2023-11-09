use super::token_type::{Literal, TokenType};

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }
}
