use crate::lox_class::LoxClass;
use std::collections::HashMap;
use crate::object::Object;
use gc_derive::{Trace, Finalize};

#[derive(Trace, Finalize, Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> LoxInstance {
        LoxInstance {
            class,
            fields: HashMap::new()
        }
    }
    pub fn get(name: &String) -> Object {
        unimplemented!()
    }
}
