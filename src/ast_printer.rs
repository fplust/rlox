use crate::expr::{
    Expr, Visitor, Binary, Grouping, Literal, Unary
};
use crate::tokentype::Literals;
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String {
        expr.accept(self)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &Binary) -> String {
        format!(
            "({} {} {})",
            expr.operator.lexeme,
            expr.left.accept(self),
            expr.right.accept(self)
        )
    }
    fn visit_grouping_expr(&self, expr: &Grouping) -> String {
        format!("(group {})", expr.expression.accept(self))
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
            expr.right.accept(self)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_print_ast() {
        use crate::tokentype::TokenType;
        use crate::token::Token;
        let minus = Token::new(TokenType::MINUS, '-'.to_string(), None, 1);
        let star = Token::new(TokenType::STAR, "*".to_string(), None, 1);
        let num1 = Literals::NUMBER(123.0);
        let num2 = Literals::NUMBER(45.67);
        let expression = Binary::new(
            Unary::new(minus, Literal::new(num1)),
            star,
            Grouping::new(Literal::new(num2)),
        );
        let printer = AstPrinter {};
        println!("{}", printer.print(&expression));
    }
}
