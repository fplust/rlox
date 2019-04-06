use crate::token::Token;
use crate::tokentype::Literals;

pub enum Expr<'a> {
    Binary(Binary<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal),
    Unary(Unary<'a>),
}

pub struct Binary<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: &'a Token,
    pub right: Box<Expr<'a>>,
}

pub struct Grouping<'a> {
    pub expression: Box<Expr<'a>>,
}

pub struct Literal {
    pub value: Literals,
}

pub struct Unary<'a> {
    pub operator: &'a Token,
    pub right: Box<Expr<'a>>,
}

impl<'a> Binary<'a> {
    pub fn new(left: Expr<'a>, operator: &'a Token, right: Expr<'a>) -> Expr<'a> {
        Expr::Binary(Binary {
            left: Box::new(left),
            operator: operator,
            right: Box::new(right),
        })
    }
}

impl<'a> Grouping<'a> {
    pub fn new(expression: Expr<'a>) -> Expr<'a> {
        Expr::Grouping(Grouping {
            expression: Box::new(expression),
        })
    }
}

impl<'a> Literal {
    pub fn new(value: Literals) -> Expr<'a> {
        Expr::Literal(Literal { value: value })
    }
}

impl<'a> Unary<'a> {
    pub fn new(operator: &'a Token, right: Expr<'a>) -> Expr<'a> {
        Expr::Unary(Unary {
            operator: operator,
            right: Box::new(right),
        })
    }
}

trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &Binary) -> T;
    fn visit_grouping_expr(&self, expr: &Grouping) -> T;
    fn visit_literal_expr(&self, expr: &Literal) -> T;
    fn visit_unary_expr(&self, expr: &Unary) -> T;
    fn accept(&self, expr: &Expr) -> T {
        match expr {
            Expr::Binary(e) => self.visit_binary_expr(e),
            Expr::Grouping(e) => self.visit_grouping_expr(e),
            Expr::Literal(e) => self.visit_literal_expr(e),
            Expr::Unary(e) => self.visit_unary_expr(e),
        }
    }
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        self.accept(expr)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        format!(
            "({} {} {})",
            expr.operator.lexeme,
            self.accept(expr.left.as_ref()),
            self.accept(expr.right.as_ref())
        )
    }
    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        format!("(group {})", self.accept(expr.expression.as_ref()))
    }
    fn visit_literal_expr(&self, expr: &Literal) -> String {
        match expr.value {
            Literals::NUMBER(n) => format!("{}", n),
            Literals::STRING(ref s) => format!("{}", s),
            Literals::BOOL(s) => format!("{}", s.unwrap()),
        }
    }
    fn visit_unary_expr(&self, expr: &Unary) -> String {
        format!(
            "({} {})",
            expr.operator.lexeme,
            self.accept(expr.right.as_ref())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_ast() {
        use crate::tokentype::TokenType;
        let minus = Token::new(TokenType::MINUS, '-'.to_string(), None, 1);
        let star = Token::new(TokenType::STAR, "*".to_string(), None, 1);
        let num1 = Literals::NUMBER(123.0);
        let num2 = Literals::NUMBER(45.67);
        let expression = Binary::new(
            Unary::new(&minus, Literal::new(num1)),
            &star,
            Grouping::new(Literal::new(num2)),
        );
        let printer = AstPrinter {};
        println!("{}", printer.print(&expression));
    }
}
