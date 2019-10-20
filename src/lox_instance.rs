use crate::lox_class::LoxClass;
use crate::object::{Object, Obj};
use gc_derive::{Finalize, Trace};
use std::collections::HashMap;
use crate::interpreter::{RTResult, RuntimeException};
use crate::expr::{Get, Set};
use std::ops::Deref;

#[derive(Trace, Finalize, Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> LoxInstance {
        LoxInstance {
            class,
            fields: HashMap::new(),
        }
    }
    pub fn get(&self, expr: &Get) -> RTResult {
        if let Some(obj) = self.fields.get(&expr.name.lexeme) {
            Ok(obj.clone())
        } else if let Some(method) = self.class.find_method(&expr.name.lexeme) {
            match method.borrow().deref() {
                Obj::Function(m) => {
                    Ok(m.bind(Object::Instance(self.clone())))
                }
                _ => unreachable!()
            }
        } else {
            Err(RuntimeException::error(
                &expr.name,
                &format!("Undefined property '{}'.", &expr.name.lexeme)
                ))
        }
    }
    pub fn set(&mut self, expr: &Set, value: Object) -> RTResult {
        self.fields.insert(expr.name.lexeme.clone(), value);
        Ok(Object::NIL())
    }
}
