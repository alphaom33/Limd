use crate::function::Callable;
use crate::value::Value;
use crate::chunk::Chunk;

use std::fmt::Display;

#[derive(Clone)]
pub enum Obj {
  Native(Native),
  Function(Function),
}

impl Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "asdf");
      return Result::Ok(());
    }
}

#[derive(Clone)]
pub struct Native {
  pub arity: u8,
  pub function: fn(&[Value]) -> Option<Value>,
}

impl Callable for Native {
  fn call(&self, args: &[Value]) -> Option<Value> {
    return (self.function)(args);
  }

  fn arity(&self) -> u8 {
    return self.arity;
  }
}

#[derive(Clone)]
pub struct Function {
  pub arity: u8,
  pub body: Chunk,
}

impl Callable for Function {
  fn call(&self, _args: &[Value]) -> Option<Value> {
    return Option::Some(Value::Nil);
  }
  
  fn arity(&self) -> u8 {
    return self.arity;
  }
}