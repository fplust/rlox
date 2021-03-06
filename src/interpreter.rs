use crate::environment::Environment;
use crate::expr;
use crate::expr::{Assign, Binary, Call, Expr, Get, Grouping, Literal, Logical, Unary, Variable, Set, This};
use crate::lox_class::LoxClass;
use crate::lox_function::{Callable, LoxFunction};
use crate::object::{Object, Obj};
use crate::stmt;
use crate::stmt::{Block, Class, Expression, Function, If, Print, Return, Stmt, Var, While};
use crate::token::Token;
use crate::tokentype::{Literals, TokenType};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

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

#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub value: Object,
}

impl ReturnValue {
    pub fn new(value: Object) -> ReturnValue {
        ReturnValue { value }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeException {
    ERROR(RuntimeError),
    RETURN(ReturnValue),
}

impl RuntimeException {
    pub fn error(token: &Token, message: &str) -> RuntimeException {
        RuntimeException::ERROR(RuntimeError::new(token, message))
    }
    pub fn return_v(value: Object) -> RuntimeException {
        RuntimeException::RETURN(ReturnValue::new(value))
    }
}

pub type RTResult = Result<Object, RuntimeException>;

static NUM_ERROR: &str = "Operands must be numbers.";
static NUM_STR_ERROR: &str = "Operands must be two numbers or two strings.";
static BOOL_ERROR: &str = "Operands must be bool.";

pub struct Interpreter {
    pub globals: Environment,
    environment: Environment,
    locals: HashMap<usize, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Environment::new();
        let env = globals.clone();
        Interpreter {
            globals,
            environment: env,
            locals: HashMap::new(),
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
    pub fn execute_block(&mut self, statements: &Vec<Stmt>, environment: Environment) -> RTResult {
        let env = self.environment.clone();
        self.environment = environment;
        // println!("current: {:?}\n", self.environment);
        // println!("statements: {:?}\n", statements);
        for statement in statements {
            if let Err(e) = self.execute(&statement) {
                self.environment = env;
                return Err(e);
            }
        }
        self.environment = env;
        // println!("up: {:?}\n", self.environment);
        Ok(Object::NIL())
    }

    pub fn resolve(&mut self, token_id: usize, depth: usize) {
        self.locals.insert(token_id, depth);
    }

    fn lookup_variable(&self, name: &Token) -> RTResult {
        let distance = self.locals.get(&name.id);
        match distance {
            Some(d) => self.environment.get_at(*d, &name.lexeme),
            None => self.globals.get(&name),
        }
    }
}


impl expr::Visitor<RTResult> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> RTResult {
        let left_o = self.evalute(&expr.left)?;
        let left_b = left_o.borrow();
        let left = left_b.deref();
        let right_o = self.evalute(&expr.right)?;
        let right_b = right_o.borrow();
        let right = right_b.deref();

        match expr.operator.token_type {
            TokenType::PLUS => match (left, right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => Ok(Object::NUMBER(l + r)),
                (Obj::STRING(l), Obj::STRING(r)) => Ok(Object::STRING(l.to_owned() + r)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_STR_ERROR)),
            },
            TokenType::MINUS => match (left, right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => Ok(Object::NUMBER(l - r)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
            },
            TokenType::SLASH => match (left, right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => Ok(Object::NUMBER(l / r)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
            },
            TokenType::STAR => match (left, right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => Ok(Object::NUMBER(l * r)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
            },
            TokenType::GREATER => match (left, right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => Ok(Object::BOOL(l > r)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
            },
            TokenType::GREATER_EQUAL => match (left, right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => Ok(Object::BOOL(l >= r)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
            },
            TokenType::LESS => match (left, right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => Ok(Object::BOOL(l < r)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
            },
            TokenType::LESS_EQUAL => match (left, right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => Ok(Object::BOOL(l <= r)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
            },
            TokenType::BANG_EQUAL => match (left, right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => {
                    // Ok(Obj::BOOL(l != r))
                    Ok(Object::BOOL((l - r).abs() >= std::f64::EPSILON))
                }
                (Obj::NIL(_), Obj::NIL(_)) => Ok(Object::BOOL(false)),
                (Obj::NIL(_), _) => Ok(Object::BOOL(true)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
            },
            TokenType::EQUAL_EQUAL => match (&left, &right) {
                (Obj::NUMBER(l), Obj::NUMBER(r)) => {
                    Ok(Object::BOOL((l - r).abs() < std::f64::EPSILON))
                }
                (Obj::STRING(l), Obj::STRING(r)) => Ok(Object::BOOL(l == r)),
                (Obj::NIL(_), Obj::NIL(_)) => Ok(Object::BOOL(true)),
                (Obj::NIL(_), _) => Ok(Object::BOOL(false)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
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
            Literals::NIL(_) => Ok(Object::NIL()),
        }
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> RTResult {
        let right = self.evalute(&expr.right)?;

        match expr.operator.token_type {
            TokenType::MINUS => match right.borrow().deref() {
                Obj::NUMBER(n) => Ok(Object::NUMBER(-n)),
                _ => Err(RuntimeException::error(&expr.operator, NUM_ERROR)),
            },
            TokenType::BANG => {
                let b = right
                    .to_bool()
                    .map_err(|_| RuntimeException::error(&expr.operator, BOOL_ERROR))?;
                Ok(Object::BOOL(!b))
            }
            _ => {
                panic!();
            }
        }
    }
    fn visit_variable_expr(&mut self, expr: &Variable) -> RTResult {
        // self.environment.borrow().get(&expr.name)
        self.lookup_variable(&expr.name)
    }
    fn visit_assign_expr(&mut self, expr: &Assign) -> RTResult {
        let value = self.evalute(&expr.value)?;
        let distance = self.locals.get(&expr.name.id);
        match distance {
            Some(d) => self.environment.assign_at(*d, &expr.name, value),
            None => self.globals.assign(&expr.name, value),
        }
        // self.environment
        //     .borrow_mut()
        //     .assign(expr.name.clone(), value)
    }
    fn visit_logical_expr(&mut self, expr: &Logical) -> RTResult {
        let left = self.evalute(&expr.left)?;
        let b = left
            .to_bool()
            .map_err(|_| RuntimeException::error(&expr.operator, ""))?;
        match expr.operator.token_type {
            TokenType::OR => {
                if b {
                    return Ok(left);
                }
            }
            TokenType::AND => {
                if !b {
                    return Ok(left);
                }
            }
            _ => return Err(RuntimeException::error(&expr.operator, "token error")),
        }
        self.evalute(&expr.right)
    }
    fn visit_call_expr(&mut self, expr: &Call) -> RTResult {
        let callee_o = self.evalute(&expr.callee)?;
        let callee_b = callee_o.borrow();
        let callee = callee_b.deref();
        let mut arguments: Vec<Object> = Vec::new();
        for argument in expr.arguments.iter() {
            arguments.push(self.evalute(&argument)?);
        }
        match callee {
            Obj::Function(func) => {
                if arguments.len() != func.arity() {
                    Err(RuntimeException::error(
                        &expr.paren,
                        format!(
                            "Expected {} arguments but got {}.",
                            func.arity(),
                            arguments.len(),
                        )
                        .as_str(),
                    ))
                } else {
                    func.call(self, arguments)
                }
            }
            Obj::Class(class) => {
                if arguments.len() != class.arity() {
                    Err(RuntimeException::error(
                        &expr.paren,
                        format!(
                            "Expected {} arguments but got {}.",
                            class.arity(),
                            arguments.len(),
                        )
                        .as_str(),
                    ))
                } else {
                    class.call(self, arguments)
                }
            }
            _ => Err(RuntimeException::error(
                &expr.paren,
                "Can only call functions and classes.",
            )),
        }
    }
    fn visit_get_expr(&mut self, expr: &Get) -> RTResult {
        let object = self.evalute(&expr.object)?;
        let o_b = object.borrow();
        if let Obj::Instance(i) = o_b.deref() {
            Ok(i.get(&expr)?)
        } else {
            Err(RuntimeException::error(
                    &expr.name,
                    "Only instances have properties."
                    ))
        }
    }
    fn visit_set_expr(&mut self, expr: &Set) -> RTResult {
        let object = self.evalute(&expr.object)?;
        let mut o_b = object.borrow_mut();
        if let Obj::Instance(ref mut i) = o_b.deref_mut() {
            let value = self.evalute(&expr.value)?;
            Ok(i.set(&expr, value)?)
        } else {
            Err(RuntimeException::error(
                    &expr.name,
                    "Only instances have properties."
                    ))
        }
    }
    fn visit_this_expr(&mut self, expr: &This) -> RTResult {
        self.lookup_variable(&expr.keyword)
    }
}

impl stmt::Visitor<RTResult> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> RTResult {
        self.evalute(&stmt.expression)
    }
    fn visit_print_stmt(&mut self, stmt: &Print) -> RTResult {
        let obj = self.evalute(&stmt.expression).unwrap();
        println!("{:?}", obj);
        Ok(Object::NIL())
    }
    fn visit_var_stmt(&mut self, stmt: &Var) -> RTResult {
        let obj = self.evalute(&stmt.initializer)?;
        self.environment.define(stmt.name.lexeme.clone(), obj);
        Ok(Object::NIL())
    }
    fn visit_block_stmt(&mut self, stmt: &Block) -> RTResult {
        self.execute_block(
            &stmt.statements,
            Environment::from_env(self.environment.clone()),
        )
    }
    fn visit_if_stmt(&mut self, stmt: &If) -> RTResult {
        let condition: bool;
        let obj = self.evalute(&stmt.condition)?;
        match obj.to_bool() {
            Ok(b) => {
                condition = b;
            }
            Err(_) => {
                return Err(RuntimeException::error(
                    &stmt.token,
                    "if statements condition type must be bool or nil",
                ));
            }
        }
        if condition {
            self.execute(&stmt.then_branch)?;
        } else if stmt.else_branch.is_some() {
            self.execute(stmt.else_branch.as_ref().unwrap())?;
        }
        Ok(Object::NIL())
    }
    fn visit_while_stmt(&mut self, stmt: &While) -> RTResult {
        loop {
            let condition = self.evalute(&stmt.condition)?;
            let b = condition.to_bool().map_err(|_| {
                RuntimeException::error(
                    &stmt.token,
                    "while statements condition type must be bool or nil",
                )
            })?;
            if !b {
                return Ok(Object::NIL());
            } else {
                self.execute(&stmt.body)?;
            }
        }
    }
    fn visit_function_stmt(&mut self, stmt: &Function) -> RTResult {
        let function = Object::Function(LoxFunction::new(stmt.clone(), self.environment.clone(), false));
        self.environment.define(stmt.name.lexeme.clone(), function);
        Ok(Object::NIL())
    }
    fn visit_return_stmt(&mut self, stmt: &Return) -> RTResult {
        let obj = self.evalute(&stmt.value)?;
        Err(RuntimeException::return_v(obj))
    }
    fn visit_class_stmt(&mut self, stmt: &Class) -> RTResult {
        let mut methods = HashMap::new();
        for method in stmt.methods.iter() {
            let name = method.name.lexeme.clone();
            let function = Object::Function(LoxFunction::new(method.clone(), self.environment.clone(), name == "init"));
            methods.insert(name, function);
        }
        let class = Object::Class(LoxClass::new(stmt.name.lexeme.clone(), methods));
        self.environment.define(stmt.name.lexeme.clone(), class);
        Ok(Object::NIL())
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
