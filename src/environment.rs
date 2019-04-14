use crate::interpreter::{Object, RTResult, RuntimeException};
use crate::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type Closure = Rc<RefCell<Environment>>;

#[derive(Debug, Clone)]
pub struct Environment {
    // TODO: 重构改为引用
    enclosing: Option<Closure>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Closure {
        Rc::new(RefCell::new(Environment {
            enclosing: None,
            values: HashMap::new(),
        }))
    }
    pub fn from_env(enclosing: Closure) -> Closure {
        Rc::new(RefCell::new(Environment {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }))
    }

    /*
    pub fn take_enclosing(&mut self) -> Box<Environment> {
        self.enclosing.take().or(Some(Environment::new())).unwrap()
    }
    */

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> RTResult {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                if self.enclosing.is_some() {
                    self.enclosing.as_ref().unwrap().borrow().get(name)
                } else {
                    Err(RuntimeException::error(
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
        } else if self.enclosing.is_some() {
            self.enclosing
                .as_mut()
                .unwrap()
                .borrow_mut()
                .assign(name, value)
        } else {
            Err(RuntimeException::error(
                &name,
                format!("Undefined variable '{}'.", name.lexeme).as_str(),
            ))
        }
    }
}
