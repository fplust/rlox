use crate::token::Token;
// use crate::tokentype::Literals;
use crate::expr::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
    If(If),
    While(While),
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

#[derive(Debug, Clone)]
pub struct If {
    pub token: Token,
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub token: Token,
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

/*
#[derive(Debug, Clone)]
pub struct For {
    pub token: Token,
    pub initializer: Option<Box<Stmt>>,
    pub condition: Option<Box<Expr>>,
    pub increment: Option<Box<Expr>>,
    pub body: Box<Stmt>,
}
*/

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

impl If {
    pub fn new(token: Token, condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Stmt {
        Stmt::If(If {
            token,
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        })
    }
}

impl While {
    pub fn new(token: Token, condition: Expr, body: Stmt) -> Stmt {
        Stmt::While(While {
            token,
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }
}

/*
impl For {
    pub fn new(token: Token, initializer: Option<Stmt>, condition: Option<Expr>, increment: Option<Expr>, body: Stmt) -> Stmt {
        Stmt::For(For {
            token,
            initializer: initializer.map(Box::new),
            condition: condition.map(Box::new),
            increment: increment.map(Box::new),
            body: Box::new(body),
        })
    }
}
*/

impl Stmt {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Stmt::Expression(e) => visitor.visit_expression_stmt(e),
            Stmt::Print(e) => visitor.visit_print_stmt(e),
            Stmt::Var(e) => visitor.visit_var_stmt(e),
            Stmt::Block(e) => visitor.visit_block_stmt(e),
            Stmt::If(e) => visitor.visit_if_stmt(e),
            Stmt::While(e) => visitor.visit_while_stmt(e),
        }
    }
}

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> T;
    fn visit_print_stmt(&mut self, stmt: &Print) -> T;
    fn visit_var_stmt(&mut self, stmt: &Var) -> T;
    fn visit_block_stmt(&mut self, stmt: &Block) -> T;
    fn visit_if_stmt(&mut self, stmt: &If) -> T;
    fn visit_while_stmt(&mut self, stmt: &While) -> T;
}
