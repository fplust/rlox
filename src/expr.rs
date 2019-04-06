use crate::token::Token;
use crate::tokentype::Literals;

pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Grouping {
    pub expression: Box<Expr>,
}

pub struct Literal {
    pub value: Literals,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

impl Grouping {
    pub fn new(expression: Expr) -> Expr {
        Expr::Grouping(Grouping {
            expression: Box::new(expression),
        })
    }
}

impl Literal {
    pub fn new(value: Literals) -> Expr {
        Expr::Literal(Literal { value: value })
    }
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Expr {
        Expr::Unary(Unary {
            operator,
            right: Box::new(right),
        })
    }
}

impl Expr {
    pub fn accept<T, V: Visitor<T>> (&self, visitor: &V) -> T {
        match self {
            Expr::Binary(e) => visitor.visit_binary_expr(e),
            Expr::Grouping(e) => visitor.visit_grouping_expr(e),
            Expr::Literal(e) => visitor.visit_literal_expr(e),
            Expr::Unary(e) => visitor.visit_unary_expr(e),
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &Binary) -> T;
    fn visit_grouping_expr(&self, expr: &Grouping) -> T;
    fn visit_literal_expr(&self, expr: &Literal) -> T;
    fn visit_unary_expr(&self, expr: &Unary) -> T;
}
