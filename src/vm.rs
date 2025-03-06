use crate::obj::Obj;
use crate::stdlib;
use std::env::var;
use std::fmt;
use std::fmt::Error;
use std::vec;
use std::slice;
use std::ops::{Add, Sub, Mul, Div};
use std::collections::{LinkedList, HashMap};

use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use enum_index::IndexEnum;

pub struct VM {
  pub stack: vec::Vec<Value>,
  pub globals: HashMap<String, Value>,
  execute_num: u8,
}

#[derive(PartialEq, Debug)]
pub enum InterpretResult {
  Ok(Option<Value>),
  CompileError,
  RuntimeError(String),
}

impl VM {
  pub fn new() -> VM {
    let globals = stdlib::get();
    VM {
      stack: vec::Vec::new(),
      globals,
      execute_num: 0,
    }
  }
  
  pub fn interpret(&mut self, chunk: Box<Chunk>) -> InterpretResult {
    self.stack.clear();
    return self.run(chunk);
  }

  fn read_byte(ip: &mut slice::Iter<u8>) -> u8 {
    return *ip.next().unwrap();
  }
  
  fn read_constant(mut ip: &mut slice::Iter<u8>, chunk: Chunk) -> Value {
    let a = Self::read_byte(&mut ip) as usize;
    return chunk.constants[a].clone();
  }

  fn get_var(&mut self, name: &str) -> InterpretResult {
    return match self.globals.get(name) {
      Some(v) => InterpretResult::Ok(Some(v.clone())),
      None => InterpretResult::RuntimeError(std::format!("{name} is not defined.")),
    }
  }

  fn resolve_var(&mut self, name: Value) -> InterpretResult {
    let Value::Symbol(s) = name else {
      return InterpretResult::RuntimeError(format!("I wanted a symbol, you idiot not {}", name));
    };

    return self.get_var(&s);
  }

  fn do_call(&mut self, to_call: Value) {

  }

  fn call_value(&mut self, mut args: &mut Vec<Value>) -> InterpretResult {
    match args.pop().expect("can't call nothin'") {
      Value::Object(o) => match *o {
        Obj::Function(f) => {
        }
        
        Obj::Native(f) => {
          if !f.varargs && args.len() != f.arity as usize {
            return InterpretResult::RuntimeError(format!("Exected {} args, but {} were given.", f.arity, args.len()))
          }
          args.reverse();
          let out = (f.function)(&mut self.globals, &mut args);

          if f.is_macro {
            self.do_call(out);
          } else {
            self.stack.push(out);
          }
        }
      },
      _ => return InterpretResult::RuntimeError("Only functions can be called.".to_owned()),
    };

    return InterpretResult::Ok(None);
  }

  fn call_macro(&mut self) -> InterpretResult {
    let Some(Value::Vector(mut args)) = self.stack.pop() else {
      return InterpretResult::RuntimeError("Expected lit??".to_owned());
    };

    let result = self.resolve_var(args.pop().expect("can't call nothin'"));
    let InterpretResult::Ok(Some(val)) =  result else {
      return result;
    };

    args.push(val);
    return self.call_value(&mut args);
  }

  fn call_vec(&mut self) -> InterpretResult {
    let Some(Value::Vector(mut args)) = self.stack.pop() else {
      return InterpretResult::RuntimeError("expected list??".to_owned());
    };
    
    return self.call_value(&mut args);
  }

  fn can_execute(&self) -> bool {
    return self.execute_num == 0;
  }

  fn run(&mut self, chunk: Box<Chunk>) -> InterpretResult {
    let mut ip = chunk.code.iter();
    while let Some(byte) = ip.next() {

      print!("          ");
      if !self.can_execute() {
        print!("x ");
      }
      for slot in &self.stack {
        print!("[ {} ]", slot);
      }
      println!();
      
      let op_code = OpCode::index_enum(*byte as usize).expect("Invalid opcode");
      'op_match: { 
        match op_code {
          OpCode::Constant => {
            let constant = Self::read_constant(&mut ip, *chunk.clone());
            self.stack.push(constant);
          },

          OpCode::GetGlobal => {
            let val = chunk.constants.get(Self::read_byte(&mut ip) as usize).expect("Byte with no constant what");

            if !self.can_execute() {
              self.stack.push(val.clone());
              break 'op_match;
            }

            let Value::Symbol(name) = val else {
              return InterpretResult::RuntimeError("you were supposed to give me a symbol, you idiot".to_owned());
            };


            let InterpretResult::Ok(result) = self.get_var(name) else {
              return InterpretResult::RuntimeError(format!("{name} is not defined"));
            };

            self.stack.push(result.unwrap());
          },
          
          OpCode::Vector => {
            let len = Self::read_byte(&mut ip);
            let mut vals: Vec<Value> = Vec::new();
            for _ in 0..len {
              vals.push(self.stack.pop().expect("Vector length didn't match number of constants."));
            }

            self.stack.push(Value::Vector(vals));
          },

          OpCode::List => {
            let len = Self::read_byte(&mut ip);
            let mut vals: LinkedList<Value> = LinkedList::new();
            for _ in 0..len {
              vals.push_front(self.stack.pop().expect(std::format!("List length was shorter than the expected {len}").as_str()));
            }
            self.stack.push(Value::List(vals));
          },
          
          OpCode::Call => {
            if self.can_execute() {
              let result = self.call_vec();
              let InterpretResult::Ok(_) = result else {
                return result;
              };
            }
          },

          OpCode::CallMacro => {
            if self.can_execute() {
              let result = self.call_macro();
              let InterpretResult::Ok(_) = result else {
                return result;
              };
            }
          },
          
          OpCode::Nexecute => self.execute_num += 1,
          OpCode::Yexecute => self.execute_num -= 1,

          OpCode::Return => return InterpretResult::Ok(self.stack.pop()),
        }
      }
    }
    return InterpretResult::Ok(self.stack.pop());
  }
}