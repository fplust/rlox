use crate::interpreter::{Interpreter, RTResult};
use crate::lox_function::Callable;
use crate::lox_instance::LoxInstance;
use crate::object::{Object, Obj};
use gc_derive::{Finalize, Trace};
use std::collections::HashMap;
use std::ops::Deref;

#[derive(Trace, Finalize, Debug, Clone)]
pub struct LoxClass {
    name: String,
    methods: HashMap<String, Object>,
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, Object>) -> LoxClass {
        LoxClass { name , methods }
    }

    pub fn find_method(&self, name: &String) -> Option<Object> {
        self.methods.get(name).cloned()
    }
}

impl Callable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> RTResult {
        let instance = Object::Instance(LoxInstance::new(self.clone()));
        if let Some(initializer) = self.find_method(&"init".to_string()) {
            if let Obj::Function(initializer) = initializer.borrow().deref() {
                let bind_method = initializer.bind(instance.clone());
                let bm = bind_method.borrow();
                if let Obj::Function(m) = bm.deref() {
                    m.call(interpreter, arguments)?;
                } else { unreachable!() }
            } else { unreachable!() }
        }
        Ok(instance)
    }

    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method(&"init".to_string()){
            if let Obj::Function(init) = initializer.borrow().deref() {
                init.arity()
            } else { unreachable!() }
        } else {
            0
        }
    }
}
