use crate::environment::Environment;
use crate::expr;
use crate::expr::{Assign, Binary, Expr, Grouping, Literal, Unary, Variable};
use crate::stmt;
use crate::stmt::{Block, Expression, Print, Stmt, Var};
use crate::token::Token;
use crate::tokentype::{Literals, TokenType};

#[derive(Debug, Clone)]
pub enum Object {
    STRING(String),
    NUMBER(f64),
    BOOL(bool),
    NIL(Option<()>),
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub fn new(token: &Token, message: &str) -> RuntimeError {
        RuntimeError {
            token: token.clone(),
            message: String::from(message),
        }
    }
}

pub type RTResult = Result<Object, RuntimeError>;

static NUM_ERROR: &str = "Operands must be numbers.";
static NUM_STR_ERROR: &str = "Operands must be two numbers or two strings.";

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(&statement);
        }
    }
    fn execute(&mut self, stmt: &Stmt) -> RTResult {
        stmt.accept(self)
    }
    fn evalute(&mut self, expr: &Expr) -> RTResult {
        expr.accept(self)
    }
    fn execute_block(&mut self, statements: &Vec<Stmt>, environment: Environment) {
        let previous = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            self.execute(&statement);
        }
        self.environment = previous;
    }
}

impl expr::Visitor<RTResult> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> RTResult {
        let left = self.evalute(&expr.left)?;
        let right = self.evalute(&expr.right)?;

        match expr.operator.token_type {
            TokenType::PLUS => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::NUMBER(l + r));
                }
                (Object::STRING(l), Object::STRING(r)) => {
                    return Ok(Object::STRING(l + r.as_str()));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_STR_ERROR));
                }
            },
            TokenType::MINUS => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::NUMBER(l - r));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            TokenType::SLASH => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::NUMBER(l / r));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            TokenType::STAR => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::NUMBER(l * r));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            TokenType::GREATER => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::BOOL(l > r));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            TokenType::GREATER_EQUAL => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::BOOL(l >= r));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            TokenType::LESS => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::BOOL(l < r));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            TokenType::LESS_EQUAL => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::BOOL(l <= r));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            TokenType::BANG_EQUAL => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::BOOL(l != r));
                }
                (Object::NIL(_), Object::NIL(_)) => {
                    return Ok(Object::BOOL(false));
                }
                (Object::NIL(_), _) => {
                    return Ok(Object::BOOL(true));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            TokenType::EQUAL_EQUAL => match (left, right) {
                (Object::NUMBER(l), Object::NUMBER(r)) => {
                    return Ok(Object::BOOL(l == r));
                }
                (Object::NIL(_), Object::NIL(_)) => {
                    return Ok(Object::BOOL(true));
                }
                (Object::NIL(_), _) => {
                    return Ok(Object::BOOL(false));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            _ => {
                panic!();
            }
        }
    }
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> RTResult {
        self.evalute(&expr.expression)
    }
    fn visit_literal_expr(&self, expr: &Literal) -> RTResult {
        match expr.value.clone() {
            Literals::NUMBER(n) => Ok(Object::NUMBER(n)),
            Literals::STRING(s) => Ok(Object::STRING(s)),
            Literals::BOOL(s) => Ok(Object::BOOL(s)),
            Literals::NIL(s) => Ok(Object::NIL(s)),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> RTResult {
        let right = self.evalute(&expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => match right {
                Object::NUMBER(n) => {
                    return Ok(Object::NUMBER(-n));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            TokenType::BANG => match right {
                Object::BOOL(n) => {
                    return Ok(Object::BOOL(!n));
                }
                _ => {
                    return Err(RuntimeError::new(&expr.operator, NUM_ERROR));
                }
            },
            _ => {
                panic!();
            }
        }
    }
    fn visit_variable_expr(&self, expr: &Variable) -> RTResult {
        self.environment.get(&expr.name)
    }
    fn visit_assign_expr(&mut self, expr: &Assign) -> RTResult {
        let value = self.evalute(&expr.value)?;
        self.environment.assign(expr.name.clone(), value)
    }
}

impl stmt::Visitor<RTResult> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> RTResult {
        self.evalute(&stmt.expression)
    }
    fn visit_print_stmt(&mut self, stmt: &Print) -> RTResult {
        let obj = self.evalute(&stmt.expression).unwrap();
        println!("{:?}", obj);
        Ok(Object::NIL(None))
    }
    fn visit_var_stmt(&mut self, stmt: &Var) -> RTResult {
        let obj = self.evalute(&stmt.initializer)?;
        self.environment.define(stmt.name.lexeme.clone(), obj);
        Ok(Object::NIL(None))
    }
    fn visit_block_stmt(&mut self, stmt: &Block) -> RTResult {
        self.execute_block(
            &stmt.statements,
            Environment::from_env(self.environment.clone()),
        );
        Ok(Object::NIL(None))
    }
}

/*
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
*/
