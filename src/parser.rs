use tokentype::{Literals, TokenType};
use token::Token;
use expr::{Expr, Binary, Literal, Grouping, Unary};
use error::report;
use std::mem;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }

    fn expression(&mut self) -> Expr<'a> {
        return self.equality();
    }

    fn equality(&mut self) -> Expr<'a> {
        let mut expr = self.comparison();
        while self.match_token(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Binary{
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr<'a> {
        let mut expr = self.addition();
        while self.match_token(vec![
                         TokenType::GREATER,
                         TokenType::GREATER_EQUAL,
                         TokenType::LESS,
                         TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.addition();
            expr = Expr::Binary(Binary{
                left: Box::new(expr),
                operator: &operator,
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn addition(&mut self) -> Expr<'a> {
        let mut expr = self.multiplication();
        while self.match_token(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.multiplication();
            expr = Expr::Binary(Binary{
                left: Box::new(expr),
                operator: &operator,
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn multiplication(&mut self) -> Expr<'a> {
        let mut expr = self.unary();
        while self.match_token(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Binary{
                left: Box::new(expr),
                operator: &operator,
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn unary(&mut self) -> Expr<'a> {
        if self.match_token(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr::Unary(Unary{
                operator: &operator,
                right: Box::new(right),
            });
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr<'a> {
        if self.match_token(vec![TokenType::FALSE]) {
            return Expr::Literal(Literal{
                value: Literals::BOOL(Some(false)),
            });
        }
        if self.match_token(vec![TokenType::TRUE]) {
            return Expr::Literal(Literal{
                value: Literals::BOOL(Some(true)),
            });
        }
        if self.match_token(vec![TokenType::NIL]) {
            return Expr::Literal(Literal{
                value: Literals::BOOL(None)
            });
        }
        if self.match_token(vec![TokenType::NUMBER, TokenType::STRING]) {
            let literal = self.previous().clone().literal.unwrap();
            return Expr::Literal(Literal{
                value: literal
            });
        }
        if self.match_token(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Expr::Grouping(Grouping{
                expression: Box::new(expr),
            });
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
        return false;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        let token = self.peek();
        return  mem::discriminant(&token_type) == mem::discriminant(&token.token_type);
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        match self.peek().token_type {
            TokenType::EOF => return true,
            _ => return false,
        }
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn previous(&self) -> &'a Token {
        return &self.tokens[self.current - 1];
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> &Token{
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
            },
            _ => {
                let w = format!(" at '{}'", token.lexeme);
                report(token.line, &w[..], message);
            },
        }
    }
}
