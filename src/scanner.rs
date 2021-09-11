use crate::error::error;
use crate::token::Token;
use crate::tokentype::{Literals, TokenType};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("and", TokenType::AND);
    m.insert("class", TokenType::CLASS);
    m.insert("if", TokenType::IF);
    m.insert("else", TokenType::ELSE);
    m.insert("true", TokenType::TRUE);
    m.insert("false", TokenType::FALSE);
    m.insert("for", TokenType::FOR);
    m.insert("fun", TokenType::FUN);
    m.insert("nil", TokenType::NIL);
    m.insert("or", TokenType::OR);
    m.insert("print", TokenType::PRINT);
    m.insert("return", TokenType::RETURN);
    m.insert("super", TokenType::SUPER);
    m.insert("this", TokenType::THIS);
    m.insert("var", TokenType::VAR);
    m.insert("while", TokenType::WHILE);
    m
});

pub struct Scanner<'a> {
    source: &'a String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        let eof = Token::new(
            self.tokens.len(),
            TokenType::EOF,
            String::from(""),
            None,
            self.line,
        );
        self.tokens.push(eof);
        &self.tokens
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),
            '!' => {
                if self.is_match('=') {
                    self.add_token(TokenType::BANG_EQUAL, None);
                } else {
                    self.add_token(TokenType::BANG, None);
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(TokenType::EQUAL_EQUAL, None);
                } else {
                    self.add_token(TokenType::EQUAL, None);
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(TokenType::LESS_EQUAL, None);
                } else {
                    self.add_token(TokenType::LESS, None);
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(TokenType::GREATER_EQUAL, None);
                } else {
                    self.add_token(TokenType::GREATER, None);
                }
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            'o' => {
                if self.is_match('r') {
                    self.add_token(TokenType::OR, None);
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => self.identifier(),
            _ => error(self.line, "Unexpected character."),
        }
    }
    fn advance(&mut self) -> char {
        let c = self.get_char(self.current);
        self.current += 1;
        c
    }
    fn add_token(&mut self, token_type: TokenType, literal: Option<Literals>) {
        let text: String = self.get_substr(self.start, self.current);
        self.tokens.push(Token::new(
            self.tokens.len(),
            token_type,
            text,
            literal,
            self.line,
        ));
    }
    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.get_char(self.current) != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_digit(&self, c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }
    fn is_alpha(&self, c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }
    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }
    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = self.get_substr(self.start, self.current);
        let keyword = KEYWORDS.get(text.as_str()).cloned();
        let token_type: TokenType;
        match keyword {
            None => token_type = TokenType::IDENTIFIER,
            Some(t) => token_type = t,
        }
        self.add_token(token_type, None);
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.get_char(self.current)
    }
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.get_char(self.current + 1)
    }
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            error(self.line, "Unexpected string.");
            return;
        }
        self.advance();
        let value = Literals::STRING(self.get_substr(self.start + 1, self.current - 1));
        self.add_token(TokenType::STRING, Some(value));
    }
    fn get_char(&self, position: usize) -> char {
        self.source.chars().nth(position).unwrap()
    }
    fn get_substr(&self, start: usize, end: usize) -> String {
        let len = end - start;
        self.source.chars().skip(start).take(len).collect()
    }
    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let value: f64 = self.get_substr(self.start, self.current).parse().unwrap();
        self.add_token(TokenType::NUMBER, Some(Literals::NUMBER(value)));
    }
}
