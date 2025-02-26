use crate::obj;
use crate::value::Value;
use std::collections::HashMap;

fn function(name: &str, arity: u8, function: fn(&mut HashMap<String, Value>, &mut [Value]) -> Value) -> (String, Value) {
  return (
    name.to_owned(),
    Value::Object(Box::new(obj::Obj::Native(obj::Native{
      arity,
      function,
    }))));
}

pub fn get() -> HashMap<String, Value> {
  return HashMap::from([
    function("def", 1, |globals, args| {
      match args[0].clone() {
        Value::Label(s) => globals.insert(s, args[1].clone()),
        _ => return Value::Nil,
      };
      return Value::Nil;
    }),
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