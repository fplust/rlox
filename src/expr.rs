use crate::token::Token;
use crate::tokentype::Literals;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
    Get(Get),
    Set(Set),
    This(This),
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: Literals,
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Box<Expr>,
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct Set {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct This {
    pub keyword: Token,
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
        Expr::Literal(Literal { value })
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

impl Variable {
    pub fn new(name: Token) -> Expr {
        Expr::Variable(Variable { name })
    }
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Expr {
        Expr::Assign(Assign {
            name,
            value: Box::new(value),
        })
    }
}

impl Logical {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Logical(Logical {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}

impl Call {
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Expr {
        Expr::Call(Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }
}

impl Get {
    pub fn new(object: Expr, name: Token) -> Expr {
        Expr::Get(Get {
            object: Box::new(object),
            name,
        })
    }
}

impl Set {
    pub fn new(object: Expr, name: Token, value: Expr) -> Expr {
        Expr::Set(Set {
            object: Box::new(object),
            name,
            value: Box::new(value),
        })
    }
}

impl This {
    pub fn new(keyword: Token) -> Expr {
        Expr::This(This {
            keyword
        })
    }
}

impl Expr {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Expr::Binary(e) => visitor.visit_binary_expr(e),
            Expr::Grouping(e) => visitor.visit_grouping_expr(e),
            Expr::Literal(e) => visitor.visit_literal_expr(e),
            Expr::Unary(e) => visitor.visit_unary_expr(e),
            Expr::Variable(e) => visitor.visit_variable_expr(e),
            Expr::Assign(e) => visitor.visit_assign_expr(e),
            Expr::Logical(e) => visitor.visit_logical_expr(e),
            Expr::Call(e) => visitor.visit_call_expr(e),
            Expr::Get(e) => visitor.visit_get_expr(e),
            Expr::Set(e) => visitor.visit_set_expr(e),
            Expr::This(e) => visitor.visit_this_expr(e),
        }
    }
}

pub trait Visitor<T> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
    fn visit_literal_expr(&self, expr: &Literal) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary) -> T;
    fn visit_variable_expr(&mut self, expr: &Variable) -> T;
    fn visit_assign_expr(&mut self, expr: &Assign) -> T;
    fn visit_logical_expr(&mut self, expr: &Logical) -> T;
    fn visit_call_expr(&mut self, expr: &Call) -> T;
    fn visit_get_expr(&mut self, expr: &Get) -> T;
    fn visit_set_expr(&mut self, expr: &Set) -> T;
    fn visit_this_expr(&mut self, expr: &This) -> T;
}
