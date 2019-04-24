use crate::object::Object;
use crate::interpreter::{Interpreter, RTResult};
use crate::lox_function::Callable;
use crate::lox_instance::LoxInstance;

#[derive(Debug, Clone)]
pub struct LoxClass {
    name: String,
}

impl LoxClass {
    pub fn new(name: String) -> LoxClass {
        LoxClass {
            name
        }
    }
}

impl Callable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> RTResult {
        Ok(Object::Instance(LoxInstance::new(self.clone())))
    }

    fn arity(&self) -> usize {
        0
    }
}
