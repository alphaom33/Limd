use crate::obj;
use crate::value::Value;
use std::collections::HashMap;

fn function(name: &str, arity: u8, varargs: bool, function: fn(&mut HashMap<String, Value>, &mut [Value]) -> Value) -> (String, Value) {
  return (
    name.to_owned(),
    Value::Object(Box::new(obj::Obj::Native(obj::Native{
      arity,
      varargs,
      function,
    }))));
}

macro_rules! binary_op {
  ($op:tt) => {
    function(stringify!($op), 0, true, |_vm, args| {
      if let Some((Value::Number(sum), rest)) = args.split_last() {
        let mut sum = *sum;
        for arg in rest {
          match arg {
            Value::Number(n) => sum = sum $op n,
            _ => return Value::Nil,
          }
        }
        return Value::Number(sum);
      }
      return Value::Nil;
     
  })};
}

pub fn get() -> HashMap<String, Value> {
  return HashMap::from([
    function("def", 2, false, |globals, args| {
      match args[0].clone() {
        Value::Label(s) => globals.insert(s, args[1].clone()),
        _ => return Value::Nil,
      };
      return Value::Nil;
    }),
    function("print", 0, true, |_vm, args| {
      for arg in args {
        print!("{} ", arg);
      }
      return Value::Nil;
    }),
    binary_op!(+),
    binary_op!(-),
    binary_op!(/),
    binary_op!(*),
  ]);
}