use crate::error::parse_error;
use crate::expr::{Assign, Binary, Call, Expr, Grouping, Literal, Logical, Unary, Variable, Get};
use crate::stmt::{Block, Expression, Function, If, Print, Return, Stmt, Var, While, Class};
use crate::token::Token;
use crate::tokentype::{Literals, TokenType};
use std::mem;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Stmt {
        if self.match_token(vec![TokenType::CLASS]) {
            return self.class_declaration();
        }
        if self.match_token(vec![TokenType::FUN]) {
            return self.function("function");
        }
        if self.match_token(vec![TokenType::VAR]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn class_declaration(&mut self) -> Stmt {
        let name = self.consume(
            TokenType::IDENTIFIER,
            "Expect class name."
        );
        self.consume(
            TokenType::LEFT_BRACE,
            "Expect '{' before class body."
        );
        let mut methods: Vec<Function> = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            if let Stmt::Function(f) = self.function("methods") {
                methods.push(f);
            } else {
                unreachable!()
            }
        }
        self.consume(
            TokenType::RIGHT_BRACE,
            "Expect '}' after class body."
        );
        Class::new(name, methods)
    }

    fn function(&mut self, kind: &str) -> Stmt {
        let name = self.consume(
            TokenType::IDENTIFIER,
            format!("Expect {} name.", kind).as_str(),
        );
        self.consume(
            TokenType::LEFT_PAREN,
            format!("Expect '(' after {} name.", kind).as_str(),
        );
        let mut parameters: Vec<Token> = Vec::new();
        if !self.check(TokenType::RIGHT_PAREN) {
            while {
                if parameters.len() >= 8 {
                    self.error(self.peek(), "Cannot have more than 8 parameters.")
                        .unwrap();
                }
                parameters.push(self.consume(TokenType::IDENTIFIER, "Expect parameter name."));
                self.match_token(vec![TokenType::COMMA])
            } {}
        }
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after parameters.");
        self.consume(
            TokenType::LEFT_BRACE,
            format!("Expect '{{' before {} body.", kind).as_str(),
        );
        let body = self.block();
        Function::new(name, parameters, body)
    }

    fn var_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenType::IDENTIFIER, "Expect variable name.");
        let initializer = if self.match_token(vec![TokenType::EQUAL]) {
            self.expression().unwrap()
        } else {
            Literal::new(Literals::NIL(None))
        };
        self.consume(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration.",
        );
        Var::new(name, initializer)
    }

    fn statement(&mut self) -> Stmt {
        if self.match_token(vec![TokenType::FOR]) {
            return self.for_statement();
        }
        if self.match_token(vec![TokenType::IF]) {
            return self.if_statement();
        }
        if self.match_token(vec![TokenType::PRINT]) {
            return self.print_statement();
        }
        if self.match_token(vec![TokenType::RETURN]) {
            return self.return_statement();
        }
        if self.match_token(vec![TokenType::WHILE]) {
            return self.while_statement();
        }
        if self.match_token(vec![TokenType::LEFT_BRACE]) {
            return Block::new(self.block());
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> Stmt {
        let token = self.previous();
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'for'.");
        let initializer = if self.match_token(vec![TokenType::SEMICOLON]) {
            None
        } else if self.match_token(vec![TokenType::VAR]) {
            Some(self.var_declaration())
        } else {
            Some(self.expression_statement())
        };
        let condition = if !self.check(TokenType::SEMICOLON) {
            self.expression().unwrap()
        } else {
            Literal::new(Literals::BOOL(true))
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after loop condition.");
        let increment = if !self.check(TokenType::RIGHT_PAREN) {
            Some(self.expression().unwrap())
        } else {
            None
        };
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after for clauses.");
        let mut body = self.statement();

        if increment.is_some() {
            body = Block::new(vec![body, Expression::new(increment.unwrap())]);
        }
        body = While::new(token, condition, body);
        if initializer.is_some() {
            body = Block::new(vec![initializer.unwrap(), body]);
        }
        body
    }

    fn if_statement(&mut self) -> Stmt {
        let token = self.previous();
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'if'.");
        let condition = self.expression().unwrap();
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after if condition.");
        let then_branch = self.statement();
        let else_branch = if self.match_token(vec![TokenType::ELSE]) {
            Some(self.statement())
        } else {
            None
        };
        If::new(token, condition, then_branch, else_branch)
    }

    fn print_statement(&mut self) -> Stmt {
        let expr = self.expression().unwrap();
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        Print::new(expr)
    }

    fn return_statement(&mut self) -> Stmt {
        let keyword = self.previous();
        let value = if !self.check(TokenType::SEMICOLON) {
            self.expression().unwrap()
        } else {
            Literal::new(Literals::NIL(None))
        };
        self.consume(TokenType::SEMICOLON, "Expect ';' after return value.");
        Return::new(keyword, value)
    }

    fn while_statement(&mut self) -> Stmt {
        let token = self.previous();
        self.consume(TokenType::LEFT_PAREN, "Expect '(' after 'while'.");
        let condition = self.expression().unwrap();
        self.consume(TokenType::RIGHT_PAREN, "Expect ')' after 'condition'.");
        let body = self.statement();
        While::new(token, condition, body)
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression().unwrap();
        self.consume(TokenType::SEMICOLON, "Expect ';' after value.");
        Expression::new(expr)
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration());
        }
        self.consume(TokenType::RIGHT_BRACE, "Expect '}' after block.");
        statements
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
        // self.equality()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.or()?;
        // let expr = self.equality()?;
        if self.match_token(vec![TokenType::EQUAL]) {
            let equals = self.previous();
            let value = self.assignment()?;
            if let Expr::Variable(e) = expr {
                let name = e.name;
                return Ok(Assign::new(name, value));
            };
            return Err(self
                .error(&equals, "Invalid assignment target.")
                .unwrap_err());
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;
        while self.match_token(vec![TokenType::OR]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Logical::new(expr, operator, right);
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;
        while self.match_token(vec![TokenType::AND]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Logical::new(expr, operator, right);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;
        while self.match_token(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Binary::new(expr, operator, right);
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.addition()?;
        while self.match_token(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.addition()?;
            expr = Binary::new(expr, operator, right);
        }
        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, String> {
        let mut expr = self.multiplication()?;
        while self.match_token(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.multiplication()?;
            expr = Binary::new(expr, operator, right);
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.match_token(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Binary::new(expr, operator, right);
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_token(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Unary::new(operator, right));
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;
        loop {
            if self.match_token(vec![TokenType::LEFT_PAREN]) {
                expr = self.finish_call(expr)?;
            } else if self.match_token(vec![TokenType::DOT]) {
                let name = self.consume(TokenType::IDENTIFIER, "Expect property name after '.'.");
                expr = Get::new(expr, name);
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(TokenType::RIGHT_PAREN) {
            while {
                if arguments.len() >= 8 {
                    self.error(self.peek(), "Cannot have more than 8 arguments.")?;
                }
                arguments.push(self.expression()?);
                self.match_token(vec![TokenType::COMMA])
            } {}
        }
        let paren = self.consume(TokenType::RIGHT_PAREN, "Expect ')' after arguments.");
        Ok(Call::new(callee, paren, arguments))
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token(vec![TokenType::FALSE]) {
            return Ok(Literal::new(Literals::BOOL(false)));
        }
        if self.match_token(vec![TokenType::TRUE]) {
            return Ok(Literal::new(Literals::BOOL(true)));
        }
        if self.match_token(vec![TokenType::NIL]) {
            return Ok(Literal::new(Literals::NIL(None)));
        }
        if self.match_token(vec![TokenType::NUMBER, TokenType::STRING]) {
            let literal = self.previous().literal.unwrap();
            return Ok(Literal::new(literal));
        }
        if self.match_token(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression.");
            return Ok(Grouping::new(expr));
        }
        if self.match_token(vec![TokenType::IDENTIFIER]) {
            let name = self.previous();
            return Ok(Variable::new(name));
        }
        Err(self.error(self.peek(), "Expect expression.").unwrap_err())
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
        self.error(self.peek(), message).unwrap();
        panic!();
    }

    fn error(&self, token: &Token, message: &str) -> Result<(), String> {
        parse_error(token, message);
        Err(String::from(message))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let TokenType::SEMICOLON = self.previous().token_type {
                return;
            }
            match self.peek().token_type {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => {
                    return;
                }
                _ => {}
            }
        }
        self.advance();
    }
}
