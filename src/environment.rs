use crate::interpreter::{Object, RTResult, RuntimeError};
use crate::token::Token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }
    pub fn from_env(enclosing: Environment) -> Environment {
        Environment {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
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
                self.enclosing.as_mut().unwrap().assign(name, value)
            } else {
                Err(RuntimeError::new(
                    &name,
                    format!("Undefined variable '{}'.", name.lexeme).as_str(),
                ))
            }
        }
    }
}
