use crate::vm::VM;
use crate::obj;
use crate::value::Value;
use std::collections::HashMap;

fn function(name: &str, arity: u8, function: fn(&mut VM, &mut [Value]) -> Value) -> (String, Value) {
  return (
    name.to_owned(),
    Value::Object(Box::new(obj::Obj::Native(obj::Native{
      arity,
      function,
    }))));
}

fn precedence(args: &[Value]) -> Value {
  for arg in args {
    match arg {
      Value::String(_) => return Value::String("".to_owned()),
      Value::Number(_) => (),
      _ => return Value::Nil,
    }
  }
  return Value::Number(0.);
}

pub fn get() -> HashMap<String, Value> {
  return HashMap::from([
    function("print", 1, |_vm, args| {
      for arg in args {
        print!("{} ", arg);
      }
      return Value::Nil;
    }),
    function("+", 2, |_vm, args| {
      let mut sum = 0.;
      for arg in args {
        match arg {
          Value::Number(n) => sum += *n,
          _ => return Value::Nil,
        }
      }
      return Value::Number(sum);
    })
  ]);
}