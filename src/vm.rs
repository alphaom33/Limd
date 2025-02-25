use crate::stdlib;
use std::vec;
use std::slice;
use std::ops::{Add, Sub, Mul, Div};
use std::collections::HashMap;

use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::obj;
use enum_index::IndexEnum;

pub struct VM<'a> {
  pub chunk: &'a Chunk,
  pub ip: slice::Iter<'a, u8>,
  pub stack: vec::Vec<Value>,
  pub globals: HashMap<String, Value>,
}

#[derive(PartialEq)]
pub enum InterpretResult {
  Ok,
  CompileError,
  RuntimeError(String),
}

impl <'a> VM<'a> {
  pub fn new(chunk: &'a Chunk) -> VM<'a> {
    let globals = stdlib::get();
    VM {
      chunk,
      ip: chunk.code.iter(),
      stack: vec::Vec::new(),
      globals,
    }
  }
  
  pub fn interpret(&mut self) -> InterpretResult {
    return self.run();
  }

  fn read_byte(ip: &mut slice::Iter<u8>) -> u8 {
    return *ip.next().unwrap();
  }
  
  fn read_constant(mut ip: &mut slice::Iter<u8>, chunk: &'a Chunk) -> Value {
    let a = Self::read_byte(&mut ip) as usize;
    return chunk.constants[a].clone();
  }

  fn run(&mut self) -> InterpretResult {
    while let Some(byte) = self.ip.next() {

      print!("          ");
      for slot in &self.stack {
        print!("[ {} ]", slot);
      }
      println!();
      
      let op_code = OpCode::index_enum(*byte as usize).expect("Invalid opcode");
      match op_code {
        OpCode::GetGlobal => {
          let name = Self::read_constant(&mut self.ip, self.chunk);
          match name {
            Value::String(s) => {
              match self.globals.get(&s) {
                Some(s) => {
                   self.stack.push(s.clone())},
                _ => return InterpretResult::RuntimeError(std::format!("{s} is not defined.")),
              }
            },
            _ => return InterpretResult::RuntimeError("??????".to_owned()),
          };
        }
        
        OpCode::Constant => {
          let constant = Self::read_constant(&mut self.ip, self.chunk);
          self.stack.push(constant);
        },

        OpCode::Call => {
          let arity = Self::read_byte(&mut self.ip);
          let mut args = vec::Vec::<Value>::new();
          for _ in 0..arity {
            args.push(self.stack.pop().unwrap());
          }
          
          match args.pop().unwrap() {
            Value::Object(o) => match *o {
              obj::Obj::Function(f) => {
              }
              
              obj::Obj::Native(f) => {
                args.reverse();
                let out = (f.function)(self, &mut args);
                self.stack.push(out);
              }
            },
            _ => return InterpretResult::RuntimeError("Only functions can be called.".to_owned()),
          };
        }
        
        OpCode::Return => return InterpretResult::Ok,
      }
    }
    return InterpretResult::Ok;
  }
}