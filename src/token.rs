use std::fmt;
use tokentype::{Literals, TokenType};

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literals>,
    line: u64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literals>, line: u64) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {:?}",
               self.token_type, self.lexeme, self.literal)
    }
}
