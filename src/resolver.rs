use crate::error::parse_error;
use crate::expr;
use crate::expr::{Assign, Binary, Call, Expr, Get, Grouping, Literal, Logical, Unary, Variable, Set, This};
use crate::interpreter::Interpreter;
use crate::stmt;
use crate::stmt::{Block, Class, Expression, Function, If, Print, Return, Stmt, Var, While};
use crate::token::Token;
use crate::tokentype::Literals;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum FunctionType {
    NONE,
    FUNCTION,
    METHOD,
    INITIALIZER,
}

#[derive(Debug, Copy, Clone)]
enum ClassType {
    NONE,
    CLASS,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_func: FunctionType,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_func: FunctionType::NONE,
            current_class: ClassType::NONE,
        }
    }
    pub fn resolves(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.resolve_s(&statement);
        }
    }

    fn resolve_s(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    fn resolve_e(&mut self, expr: &Expr) {
        expr.accept(self);
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if !self.scopes.is_empty() {
            let len = self.scopes.len();
            let scope = self.scopes.get_mut(len - 1).unwrap();
            if scope.contains_key(&name.lexeme) {
                parse_error(
                    name,
                    "Variable with this name already declared in this scope.",
                );
                panic!();
            }
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if !self.scopes.is_empty() {
            let len = self.scopes.len();
            let scope = self.scopes.get_mut(len - 1).unwrap();
            scope.insert(name.lexeme.clone(), true);
        }
    }

    fn resolve_local(&mut self, name: &Token) {
        for (i, item) in self.scopes.iter().rev().enumerate() {
            if item.contains_key(&name.lexeme) {
                self.interpreter.resolve(name.id, i);
                return;
            }
        }
    }

    fn resolve_function(&mut self, function: &Function, f_type: FunctionType) {
        let enclosing_func = self.current_func;
        self.current_func = f_type;
        self.begin_scope();
        for param in function.params.iter() {
            self.declare(&param);
            self.define(&param);
        }
        self.resolves(&function.body);
        self.end_scope();
        self.current_func = enclosing_func;
    }
}

impl<'a> expr::Visitor<()> for Resolver<'a> {
    fn visit_variable_expr(&mut self, expr: &Variable) {
        // println!("{:?} {:?}", self.scopes, expr);
        if !self.scopes.is_empty()
            && !*self
                .scopes
                .last()
                .unwrap()
                .get(&expr.name.lexeme)
                .or(Some(&true))
                .unwrap()
        {
            parse_error(
                &expr.name,
                "Cannot read local variable in its own initializer.",
            );
            panic!();
        }
        self.resolve_local(&expr.name);
    }
    fn visit_binary_expr(&mut self, expr: &Binary) {
        self.resolve_e(&expr.left);
        self.resolve_e(&expr.right);
    }
    fn visit_call_expr(&mut self, expr: &Call) {
        self.resolve_e(&expr.callee);
        for argument in expr.arguments.iter() {
            self.resolve_e(&argument);
        }
    }
    fn visit_grouping_expr(&mut self, expr: &Grouping) {
        self.resolve_e(&expr.expression);
    }
    fn visit_literal_expr(&self, _expr: &Literal) {}
    fn visit_logical_expr(&mut self, expr: &Logical) {
        self.resolve_e(&expr.left);
        self.resolve_e(&expr.right);
    }
    fn visit_unary_expr(&mut self, expr: &Unary) {
        self.resolve_e(&expr.right);
    }
    fn visit_assign_expr(&mut self, expr: &Assign) {
        self.resolve_e(&expr.value);
        self.resolve_local(&expr.name);
    }
    fn visit_get_expr(&mut self, expr: &Get) {
        self.resolve_e(&expr.object);
    }
    fn visit_set_expr(&mut self, expr: &Set) {
        self.resolve_e(&expr.value);
        self.resolve_e(&expr.object);
    }
    fn visit_this_expr(&mut self, expr: &This) {
        if let ClassType::NONE = self.current_class  {
            panic!("Cannot use 'this' outside of a class.");
        }
        self.resolve_local(&expr.keyword);
    }
}

impl<'a> stmt::Visitor<()> for Resolver<'a> {
    fn visit_block_stmt(&mut self, stmt: &Block) {
        self.begin_scope();
        self.resolves(&stmt.statements);
        self.end_scope();
    }
    fn visit_var_stmt(&mut self, stmt: &Var) {
        self.declare(&stmt.name);
        self.resolve_e(&stmt.initializer);
        self.define(&stmt.name);
    }
    fn visit_function_stmt(&mut self, stmt: &Function) {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(stmt, FunctionType::FUNCTION);
    }
    fn visit_expression_stmt(&mut self, stmt: &Expression) {
        self.resolve_e(&stmt.expression);
    }
    fn visit_if_stmt(&mut self, stmt: &If) {
        self.resolve_e(&stmt.condition);
        self.resolve_s(&stmt.then_branch);
        if stmt.else_branch.is_some() {
            self.resolve_s(&stmt.else_branch.as_ref().unwrap());
        }
    }
    fn visit_print_stmt(&mut self, stmt: &Print) {
        self.resolve_e(&stmt.expression);
    }
    fn visit_return_stmt(&mut self, stmt: &Return) {
        match self.current_func {
            FunctionType::FUNCTION | FunctionType::METHOD => self.resolve_e(&stmt.value),
            FunctionType::INITIALIZER => {
                if let Expr::Literal(l) = stmt.value.as_ref() {
                    if let Literals::NIL(_) = l.value {
                        self.resolve_e(&stmt.value);
                        return;
                    }
                }
                panic!("Can not return a value from an initializer.")
            }
            FunctionType::NONE => {
                parse_error(&stmt.keyword, "Cannot return from top-level code.");
                panic!();
            }
        }
    }
    fn visit_while_stmt(&mut self, stmt: &While) {
        self.resolve_e(&stmt.condition);
        self.resolve_s(&stmt.body);
    }
    fn visit_class_stmt(&mut self, stmt: &Class) {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::CLASS;
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.begin_scope();
        let last = self.scopes.len() - 1;
        self.scopes[last].insert("this".to_string(), true);

        for method in stmt.methods.iter() {
            if method.name.lexeme == "init" {
                self.resolve_function(method, FunctionType::INITIALIZER);
            } else {
                self.resolve_function(method, FunctionType::METHOD);
            }
        }
        self.end_scope();
        self.current_class = enclosing_class;
    }
}
