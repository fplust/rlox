use crate::error::report;
use crate::expr::{Binary, Expr, Grouping, Literal, Unary};
use std::mem;
use crate::token::Token;
use crate::tokentype::{Literals, TokenType};

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_token(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Binary::new(expr, operator, right);
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.addition();
        while self.match_token(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.addition();
            expr = Binary::new(expr, operator, right);
        }
        expr
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();
        while self.match_token(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.multiplication();
            expr = Binary::new(expr, operator, right);
        }
        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_token(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Binary::new(expr, operator, right);
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Unary::new(operator, right);
        }
        self.primary()
    }

    fn primary(&mut self) -> Expr {
        if self.match_token(vec![TokenType::FALSE]) {
            return Literal::new(Literals::BOOL(Some(false)));
        }
        if self.match_token(vec![TokenType::TRUE]) {
            return Literal::new(Literals::BOOL(Some(true)));
        }
        if self.match_token(vec![TokenType::NIL]) {
            return Literal::new(Literals::BOOL(None));
        }
        if self.match_token(vec![TokenType::NUMBER, TokenType::STRING]) {
            let literal = self.previous().literal.unwrap();
            return Literal::new(literal);
        }
        if self.match_token(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Grouping::new(expr);
        }
        self.error(self.peek(), "Expect expression.");
        panic!();
    }

    fn match_token(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        let token = self.peek();
        mem::discriminant(&token_type) == mem::discriminant(&token.token_type)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek().token_type {
            TokenType::EOF => true,
            _ => false,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }
        self.error(self.peek(), message);
        panic!();
    }

    fn error(&self, token: &Token, message: &str) {
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
}
