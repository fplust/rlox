use crate::environment::Environment;
use crate::expr;
use crate::expr::{Assign, Binary, Expr, Grouping, Literal, Unary, Variable, Logical};
use crate::stmt;
use crate::stmt::{Block, Expression, Print, Stmt, Var, If, While};
use crate::token::Token;
use crate::tokentype::{Literals, TokenType};

#[derive(Debug, Clone)]
pub enum Object {
    STRING(String),
    NUMBER(f64),
    BOOL(bool),
    NIL(Option<()>),
}

impl Object {
    fn to_bool(&self) -> Result<bool, ()> {
        match self {
            Object::BOOL(b) => Ok(b.clone()),
            Object::NIL(_) => Ok(false),
            _ => Err(()),
        }
    }
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
static BOOL_ERROR: &str = "Operands must be bool.";

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
            self.execute(&statement).unwrap();
        }
    }
    fn execute(&mut self, stmt: &Stmt) -> RTResult {
        stmt.accept(self)
    }
    fn evalute(&mut self, expr: &Expr) -> RTResult {
        expr.accept(self)
    }
    fn execute_block(&mut self, statements: &Vec<Stmt>, environment: Environment) -> RTResult {
        self.environment = environment;
        for statement in statements {
            self.execute(&statement)?;
        }
        self.environment = self.environment.get_enclosing();
        Ok(Object::NIL(None))
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
            TokenType::BANG => {
                let b = right.to_bool().map_err(|_| RuntimeError::new(&expr.operator, BOOL_ERROR))?;
                return Ok(Object::BOOL(!b))
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
    fn visit_logical_expr(&mut self, expr: &Logical) -> RTResult {
        let left = self.evalute(&expr.left)?;
        let b = left.to_bool().map_err(|_| RuntimeError::new(&expr.operator, ""))?;
        match expr.operator.token_type {
            TokenType::OR => { if b { return Ok(left); } },
            TokenType::AND => { if !b { return Ok(left); } },
            _ => { return Err(RuntimeError::new(&expr.operator, "token error")) },
        }
        self.evalute(&expr.right)
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
            Environment::from_env(self.environment.clone())
        )
    }
    fn visit_if_stmt(&mut self, stmt: &If) -> RTResult {
        let condition: bool;
        let obj = self.evalute(&stmt.condition)?;
        match obj.to_bool() {
            Ok(b) => { condition = b; },
            Err(_) => { return Err(RuntimeError::new(&stmt.token, "if statements condition type must be bool or nil")); }
        }
        if condition {
            self.execute(&stmt.then_branch)?;
        } else if stmt.else_branch.is_some() {
            self.execute(stmt.else_branch.as_ref().unwrap())?;
        }
        Ok(Object::NIL(None))
    }
    fn visit_while_stmt(&mut self, stmt: &While) -> RTResult {
        loop {
            let condition = self.evalute(&stmt.condition)?;
            let b = condition.to_bool().map_err(
                |_| RuntimeError::new(&stmt.token, "while statements condition type must be bool or nil"))?;
            if !b {
                return Ok(Object::NIL(None));
            } else {
                self.execute(&stmt.body)?;
            }
        }
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
