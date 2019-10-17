use crate::object::Object;
use crate::interpreter::{RTResult, RuntimeException};
use crate::token::Token;
use std::collections::HashMap;
// use std::borrow::{Borrow, BorrowMut};
use gc::{Gc, GcCell};
use gc_derive::{Trace, Finalize};


#[derive(Trace, Finalize, Debug)]
pub struct Environment {
    // TODO: 重构改为引用
    enclosing: Option<GcEnv>,
    values: HashMap<String, Object>,
}

pub type GcEnv = Gc<GcCell<Environment>>;

impl Environment {
    pub fn new() -> GcEnv {
        Gc::new(GcCell::new(Environment {
            enclosing: None,
            values: HashMap::new(),
        }))
    }
    pub fn from_env(env: GcEnv) -> GcEnv {
        Gc::new(GcCell::new(Environment {
            enclosing: Some(env),
            values: HashMap::new(),
        }))
    }

    pub fn get_enclosing(&self) -> Option<GcEnv> {
        match &self.enclosing {
            Some(env) => Some(env.clone()),
            None => None
        }
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

    pub fn get_at(&self, distance: usize, name: &String) -> RTResult {
        // println!("{} distance: {}", name, distance);
        // println!("{:?}", self);
        if distance == 0 {
            // 可能是bug, 打印 hashmap 出错
            // println!("{:?}", self.values);
            Ok(self.values.get(name).unwrap().clone())
        } else {
            Ok(self
                .ancestor(distance)
                .borrow()
                .values
                .get(name)
                .unwrap()
                .clone())
        }
    }

    fn ancestor(&self, distance: usize) -> GcEnv {
        let mut environment = self.enclosing.as_ref().unwrap().clone();
        for _ in 1..distance {
            environment = environment
                .clone()
                .borrow()
                .enclosing.as_ref()
                .unwrap()
                .clone();
        }
        environment
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> RTResult {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            Ok(value)
        }  else if self.enclosing.is_some() {
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

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: Object) -> RTResult {
        if distance == 0 {
            self.values.insert(name.lexeme.clone(), value.clone());
        } else {
            self.ancestor(distance)
                .borrow_mut()
                .values
                .insert(name.lexeme.clone(), value.clone());
        }
        Ok(value)
    }
}
