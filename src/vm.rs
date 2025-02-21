use crate::function::Callable;
use std::vec;
use std::slice;
use std::ops::{Add, Sub, Mul, Div};

use crate::chunk::{Chunk, OpCode};
use crate::value::Value;
use crate::obj;
use enum_index::IndexEnum;

pub struct VM<'a> {
  pub chunk: &'a Chunk,
  pub ip: slice::Iter<'a, u8>,
  pub stack: vec::Vec<Value>,
}

pub enum InterpretResult {
  Ok,
  CompileError,
  RuntimeError,
}

impl <'a> VM<'a> {
  pub fn new(chunk: &'a Chunk) -> VM<'a> {
    VM {
      chunk,
      ip: chunk.code.iter(),
      stack: vec::Vec::new(),
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
        
        OpCode::OpConstant => {
          let constant = Self::read_constant(&mut self.ip, self.chunk);
          self.stack.push(constant);
        },

        OpCode::OpFunction => {
          let function: Box<dyn Callable> = match self.stack.pop().unwrap() {
            Value::Object(o) => match o {
              obj::Obj::Function(f) => Box::new(f),
              obj::Obj::Native(f) => Box::new(f),
            },
            _ => return InterpretResult::RuntimeError,
          };
          let mut args: Vec<Value> = Vec::new();
          for _ in 0..function.arity() {
            args.push(self.stack.pop().unwrap());
          }
          match function.call(&args) {
            Some(s) => self.stack.push(s.clone()),
            None => return InterpretResult::RuntimeError,
          }
        }
        
        OpCode::OpReturn => return InterpretResult::Ok,
      }
    }
    return InterpretResult::Ok;
  }
}