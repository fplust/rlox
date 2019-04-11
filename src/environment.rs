use crate::interpreter::{Object, RTResult, RuntimeError};
use crate::token::Token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    // TODO: 重构改为引用
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Box<Environment> {
        Box::new(Environment {
            enclosing: None,
            values: HashMap::new(),
        })
    }
    pub fn from_env(enclosing: Box<Environment>) -> Box<Environment> {
        Box::new(Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        })
    }

    pub fn take_enclosing(&mut self) -> Box<Environment> {
        self.enclosing.take().unwrap()
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> RTResult {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                if self.enclosing.is_some() {
                    return self.enclosing.as_ref().unwrap().get(name);
                } else {
                    Err(RuntimeError::new(
                        &name,
                        format!("Undefined variable '{}'.", name.lexeme).as_str(),
                    ))
                }
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: Object) -> RTResult {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value.clone());
            Ok(value)
        } else {
            if self.enclosing.is_some() {
                let r = self.enclosing.as_mut().unwrap().assign(name, value);
                return r
            } else {
                Err(RuntimeError::new(
                    &name,
                    format!("Undefined variable '{}'.", name.lexeme).as_str(),
                ))
            }
        }
    }
}
