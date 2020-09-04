use crate::eval::{Env, Object};
use std::cell::RefCell;
use std::rc::Rc;

pub fn init(env: &mut Env) {
    env.set(
        "print".to_string(),
        Rc::new(RefCell::new(Object::NativeFunction {
            function: |args| {
                for arg in args {
                    match &*arg.borrow() {
                        Object::Nil => println!("nil"),
                        Object::Int { value } => println!("{}", value),
                        Object::String { value } => {
                            println!("{}", value.iter().collect::<String>())
                        }
                        _ => unimplemented!(),
                    }
                }
                Rc::new(RefCell::new(Object::Nil))
            },
        })),
    );
}
