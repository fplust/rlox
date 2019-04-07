use crate::token::Token;
// use crate::tokentype::Literals;
use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Expression {
    pub fn new(expression: Expr) -> Stmt {
        Stmt::Expression(Expression {
            expression: Box::new(expression),
        })
    }
}

impl Print {
    pub fn new(expression: Expr) -> Stmt {
        Stmt::Print(Print {
            expression: Box::new(expression),
        })
    }
}

impl Var {
    pub fn new(name: Token, initializer: Expr) -> Stmt {
        Stmt::Var(Var {
            name,
            initializer: Box::new(initializer),
        })
    }
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Stmt {
        Stmt::Block(Block { statements })
    }
}

impl Stmt {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Stmt::Expression(e) => visitor.visit_expression_stmt(e),
            Stmt::Print(e) => visitor.visit_print_stmt(e),
            Stmt::Var(e) => visitor.visit_var_stmt(e),
            Stmt::Block(e) => visitor.visit_block_stmt(e),
        }
    }
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> T;
    fn visit_print_stmt(&mut self, stmt: &Print) -> T;
    fn visit_var_stmt(&mut self, stmt: &Var) -> T;
    fn visit_block_stmt(&mut self, stmt: &Block) -> T;
}
