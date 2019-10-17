use crate::interpreter::{RTResult, RuntimeException};
use crate::object::Object;
use crate::token::Token;
use std::collections::HashMap;
// use std::borrow::{Borrow, BorrowMut};
use gc::{Gc, GcCell};
use gc_derive::{Finalize, Trace};
use std::ops::Deref;

#[derive(Trace, Finalize, Debug)]
pub struct Env {
    enclosing: Option<Environment>,
    values: HashMap<String, Object>,
}

type GcEnv = Gc<GcCell<Env>>;

#[derive(Trace, Finalize, Clone, Debug)]
pub struct Environment {
    env: GcEnv,
}

impl Deref for Environment {
    type Target = GcCell<Env>;
    #[inline]
    fn deref(&self) -> &GcCell<Env> {
        &self.env
    }
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            env: Gc::new(GcCell::new(Env {
                enclosing: None,
                values: HashMap::new(),
            })),
        }
    }
    pub fn from_env(env: Environment) -> Environment {
        Environment {
            env: Gc::new(GcCell::new(Env {
                enclosing: Some(env),
                values: HashMap::new(),
            })),
        }
    }

    pub fn get_enclosing(&self) -> Option<Environment> {
        match &self.borrow().enclosing {
            Some(env) => Some(env.clone()),
            None => None,
        }
    }

    /*
    pub fn take_enclosing(&mut self) -> Box<Environment> {
        self.enclosing.take().or(Some(Environment::new())).unwrap()
    }
    */

    pub fn define(&mut self, name: String, value: Object) {
        self.borrow_mut().values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> RTResult {
        match self.borrow().values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                if self.borrow().enclosing.is_some() {
                    self.borrow().enclosing.as_ref().unwrap().get(name)
                } else {
                    Err(RuntimeException::error(
                        &name,
                        format!("Undefined variable '{}'.", name.lexeme).as_str(),
                    ))
                }
            }
        }
    }

    pub fn get_at(&self, distance: usize, name: &String) -> RTResult {
        // println!("{} distance: {}", name, distance);
        // println!("{:?}", self);
        if distance == 0 {
            // 可能是bug, 打印 hashmap 出错
            // println!("{:?}", self.values);
            Ok(self.borrow().values.get(name).unwrap().clone())
        } else {
            Ok(self
                .ancestor(distance)
                .borrow()
                .values
                .get(name)
                .unwrap()
                .clone()
            )
        }
    }

    fn ancestor(&self, distance: usize) -> Environment {
        let mut environment = self.borrow().enclosing.as_ref().unwrap().clone();
        for _ in 1..distance {
            environment = environment
                .clone()
                .borrow()
                .enclosing
                .as_ref()
                .unwrap()
                .clone();
        }
        environment
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> RTResult {
        if self.borrow().values.contains_key(&name.lexeme) {
            self.borrow_mut()
                .values
                .insert(name.lexeme.clone(), value.clone());
            Ok(value)
        } else if self.borrow().enclosing.is_some() {
            self.borrow_mut()
                .enclosing
                .as_mut()
                .unwrap()
                .assign(name, value)
        } else {
            Err(RuntimeException::error(
                &name,
                format!("Undefined variable '{}'.", name.lexeme).as_str(),
            ))
        }
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) -> RTResult {
        if distance == 0 {
            self.borrow_mut()
                .values
                .insert(name.lexeme.clone(), value.clone());
        } else {
            self.ancestor(distance)
                .borrow_mut()
                .values
                .insert(name.lexeme.clone(), value.clone());
        }
        Ok(value)
    }
}
