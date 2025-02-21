use crate::value::Value;

pub trait Callable {
  fn call(&self, _args: &[Value]) -> Option<Value>;
  fn arity(&self) -> u8;
}