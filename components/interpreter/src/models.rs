use crate::{InterpreterResult, State};
use html::HTMLElement;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub enum AstryxNode {
    HTMLElement(HTMLElement),
    Root,
}

pub type BuiltinFunction = fn(Rc<RefCell<State>>) -> InterpreterResult<Object>;

#[derive(Clone, Debug)]
pub enum Object {
    String(String),
    // FunctionLiteral {
    //     params: Vec<String>,
    //     statements: Vec<Statement<'a>>,
    // },
    BuiltinFunction(BuiltinFunction),
    Array(Vec<Object>),
    Map(HashMap<String, Object>),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::String(s) => format!("{:?}", s),
            // Object::FunctionLiteral { params, statements } => {
            //     format!("{:?}{:?}", params, statements)
            // }
            Object::BuiltinFunction(f) => format!("builtin_{:?}()", f),
            Object::Array(v) => format!(
                "[{}]",
                v.iter()
                    .map(Object::inspect)
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Object::Map(_) => unimplemented!(),
        }
    }
}

impl ToString for Object {
    fn to_string(&self) -> String {
        match self {
            Object::BuiltinFunction(_) => format!("__BuiltinFunction"),
            Object::String(s) => s.clone(),
            // Object::FunctionLiteral { params, statements } => format!("__FunctionLiteral"),
            Object::Array(_) => unimplemented!(),
            Object::Map(_) => unimplemented!(),
        }
    }
}

impl Into<String> for Object {
    fn into(self) -> String {
        match self {
            Object::String(s) => s,
            // Object::FunctionLiteral { params, statements } => format!("({:?})", params),
            Object::BuiltinFunction(_) => unimplemented!(),
            Object::Array(_) => unimplemented!(),
            Object::Map(_) => unimplemented!(),
        }
    }
}
