use crate::token::Token;
use crate::tokentype::TokenType;

fn report(line: u64, w: &str, message: &str) {
    println!("[line {}] Error{}: {}", line, w, message);
}

pub fn error(line: u64, message: &str) {
    report(line, "", message);
}

pub fn parse_error(token: &Token, message: &str) {
    match token.token_type {
        TokenType::EOF => {
            report(token.line, " at end", message);
        }
        _ => {
            let w = format!(" at '{}'", token.lexeme);
            report(token.line, &w[..], message);
        }
    }
}
