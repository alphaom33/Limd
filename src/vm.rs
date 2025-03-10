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

pub type InterpretResult = Result<Value, String>;

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
      Some(v) => InterpretResult::Ok(v.clone()),
      None => InterpretResult::Err(std::format!("{name} is not defined.")),
    }
  }

  fn resolve_var(&mut self, name: Value) -> InterpretResult {
    let Value::Symbol(s) = name else {
      return InterpretResult::Err(format!("I wanted a symbol, you idiot not {}", name));
    };

    return self.get_var(&s);
  }

  pub fn do_call(&mut self, to_call: Value) -> InterpretResult {
    match to_call {
      Value::Vector(v) => {
        let mut args = Vec::new();
        for arg in v {
          let result = self.do_call(arg);
          let InterpretResult::Ok(val) = result else {
            return result;
          };
          args.push(val);
        };
        return self.call_value(&mut args);
      },
      Value::Symbol(s) => self.get_var(&s),
      _ => return InterpretResult::Ok(to_call),
    }
  }

  fn call_value(&mut self, mut args: &mut Vec<Value>) -> InterpretResult {
    match args.pop().expect("can't call nothin'") {
      Value::Object(o) => match *o {
        Obj::Function(f) => {
        }
        
        Obj::Native(f) => {
          if !f.varargs && args.len() != f.arity as usize {
            return InterpretResult::Err(format!("Exected {} args, but {} were given.", f.arity, args.len()))
          }
          args.reverse();
          return (f.function)(self, &mut args);

        }
      },
      _ => return InterpretResult::Err("Only functions can be called.".to_owned()),
    };

    return InterpretResult::Ok(Value::Nil);
  }

  fn call_macro(&mut self) -> InterpretResult {
    let Some(Value::Vector(mut args)) = self.stack.pop() else {
      return InterpretResult::Err("Expected list??".to_owned());
    };

    let result = self.resolve_var(args.pop().expect("can't call nothin'"));
    let InterpretResult::Ok(val) =  result else {
      return result;
    };

    args.push(val);
    let result  = self.call_value(&mut args);
    if let InterpretResult::Ok(out) = result {
      return self.do_call(out);
    } else {
      return result;
    }
  }

  fn call_vec(&mut self) -> InterpretResult {
    let Some(Value::Vector(mut args)) = self.stack.pop() else {
      return InterpretResult::Err("expected list??".to_owned());
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
              return InterpretResult::Err("you were supposed to give me a symbol, you idiot".to_owned());
            };


            let InterpretResult::Ok(result) = self.get_var(name) else {
              return InterpretResult::Err(format!("{name} is not defined"));
            };

            self.stack.push(result);
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
              if let InterpretResult::Ok(v) = result {
                self.stack.push(v);
              } else {
                return result;
              };
            }
          },

          OpCode::CallMacro => {
            if self.can_execute() {
              let result = self.call_macro();
              let InterpretResult::Ok(val) = result else {
                return result;
              };
              self.stack.push(val);
            }
          },
          
          OpCode::Nexecute => self.execute_num += 1,
          OpCode::Yexecute => self.execute_num -= 1,

          OpCode::Return => return InterpretResult::Ok(self.stack.pop().expect("Nothing on stack??")),
        }
      }
    }
    return InterpretResult::Err("Probably should be a return somewhere".to_owned())
  }
}