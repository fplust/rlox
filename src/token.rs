use crate::tokentype::{Literals, TokenType};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Token {
    pub id: usize,
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literals>,
    pub line: u64,
}

impl Token {
    pub fn new(
        id: usize,
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literals>,
        line: u64,
    ) -> Token {
        Token {
            id,
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
