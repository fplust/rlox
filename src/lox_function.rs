use crate::environment::Environment;
use crate::interpreter::{Interpreter, RTResult, RuntimeException};
use crate::object::Object;
use crate::stmt::Function;
use gc_derive::{Finalize, Trace};
use std::fmt;

pub trait Callable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> RTResult;
    fn arity(&self) -> usize;
}

// #[derive(Debug, Clone)]
// enum FuncClosure {
//     STRONG(Closure),
//     WEAK(WeakClosure),
// }

// impl FuncClosure {
//     pub fn to_strong(&self) -> Closure {
//         match self {
//             FuncClosure::STRONG(e) => e.clone(),
//             FuncClosure::WEAK(e) => e.upgrade().unwrap(),
//         }
//     }
// }

#[derive(Trace, Finalize, Clone)]
pub struct LoxFunction {
    #[unsafe_ignore_trace]
    declaration: Function, // 该项不会含有gc管理的对象
    closure: Environment,
    is_initializer: bool,
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.declaration.name)
    }
}

// impl Clone for LoxFunction {
//     fn clone(&self) -> Self {
//         let closure = self.closure.to_strong();
//         LoxFunction {
//             declaration: self.declaration.clone(),
//             closure: FuncClosure::STRONG(closure)
//         }
//     }
// }

impl LoxFunction {
    pub fn new(declaration: Function, env: Environment, is_initializer: bool) -> LoxFunction {
        LoxFunction {
            declaration,
            closure: env,
            is_initializer,
        }
    }

    pub fn bind(&self, instance: Object) -> Object {
        let mut env = Environment::from_env(self.closure.clone());
        env.define("this".to_string(), instance);
        Object::Function(LoxFunction::new(self.declaration.clone(), env, self.is_initializer))  // clone declaration is expensive
    }
}

// impl Drop for LoxFunction {
//     fn drop(&mut self) {
//         println!("drop func");
//     }
// }

impl Callable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> RTResult {
        let mut environment = Environment::from_env(self.closure.clone());
        for (i, param) in self.declaration.params.iter().enumerate() {
            environment.define(param.lexeme.clone(), arguments[i].clone());
        }
        // println!("func: {:?}\n", environment);
        match interpreter.execute_block(&self.declaration.body, environment) {
            Ok(obj) => {
                if self.is_initializer {
                    self.closure.get_at(0, &"this".to_string())
                } else {
                    Ok(obj)
                }
            }
            Err(exception) => match exception {
                RuntimeException::RETURN(rv) => {
                    if self.is_initializer {
                        self.closure.get_at(0, &"this".to_string())
                    } else {
                        Ok(rv.value)
                    }
                }
                _ => Err(exception),
            },
        }
        // Ok(Object::NIL(None))
    }
    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}
