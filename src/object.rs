use crate::lox_class::LoxClass;
use crate::lox_function::LoxFunction;
use crate::lox_instance::LoxInstance;
use gc::{Gc, GcCell};
use gc_derive::{Finalize, Trace};
use std::ops::Deref;


#[derive(Trace, Finalize, Debug, Clone)]
pub struct Object {
    obj: GcObj
}

macro_rules! ret {
    ($e:expr) => (Object {
        obj: Gc::new(GcCell::new($e))
    });
}

impl Object {
    pub fn STRING(s: String) -> Self {
        ret!(Obj::STRING(s))
    }
    pub fn NUMBER(s: f64) -> Self {
        ret!(Obj::NUMBER(s))
    }
    pub fn BOOL(s: bool) -> Self {
        ret!(Obj::BOOL(s))
    }
    pub fn NIL() -> Self {
        ret!(Obj::NIL(None))
    }
    pub fn Function(s: LoxFunction) -> Self {
        ret!(Obj::Function(s))
    }
    pub fn Class(s: LoxClass) -> Self {
        ret!(Obj::Class(s))
    }
    pub fn Instance(s: LoxInstance) -> Self {
        ret!(Obj::Instance(s))
    }

    pub fn to_bool(&self) -> Result<bool, ()> {
        match self.obj.borrow().deref() {
            Obj::BOOL(b) => Ok(*b),
            Obj::NIL(_) => Ok(false),
            _ => Err(()),
        }
    }
}

impl Deref for Object {
    type Target = GcCell<Obj>;
    #[inline]
    fn deref(&self) -> &GcCell<Obj> {
        &self.obj
    }
}

type GcObj = Gc<GcCell<Obj>>;

#[derive(Trace, Finalize, Debug)]
pub enum Obj {
    STRING(String),
    NUMBER(f64),
    BOOL(bool),
    NIL(Option<()>),
    Function(LoxFunction),
    Class(LoxClass),
    Instance(LoxInstance),
}
