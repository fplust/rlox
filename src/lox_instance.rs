use crate::lox_class::LoxClass;
use crate::object::Object;
use gc_derive::{Finalize, Trace};
use std::collections::HashMap;

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
    pub fn get(name: &String) -> Object {
        unimplemented!()
    }
}
