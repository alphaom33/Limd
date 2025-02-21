use crate::value::Value;
use crate::chunk::Chunk;
use std::fmt::Display;

pub enum Obj {
  Native(Native),
  Function(Function),
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self);
      return Result::Ok(());
    }
}

pub struct Native {
  pub function: fn([Value]) -> Value,
}

pub struct Function {
  pub arity: u8,
  pub body: Chunk,
}

impl Function {
  pub fn call(&self, _args: &[Value]) -> Value {
    return Value::Nil;    
  }
}