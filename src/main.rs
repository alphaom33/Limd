use std::fs;
use std::env;
use std::io::{Write, stdin, stdout};

extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;

mod scanner;
mod chunk;
mod debug;
mod value;
mod vm;
mod obj;
mod function;

fn main() {
  let mut chunk = chunk::Chunk::new();

  let c = chunk.add_constant(value::Value::Object(obj::Obj::Native(obj::Native{
    arity: 2,
    function: |args| {
      let a = args[0].clone();
      let b = args[1].clone();
      
      let sum = match (a, b) {
        (value::Value::Number(a), value::Value::Number(b)) => a + b,
        _ => return Option::None,
      };
        
      return Option::Some(value::Value::Number(sum));
    }
  })));
  
  let a = chunk.add_constant(value::Value::Number(1.2));
  let b = chunk.add_constant(value::Value::Number(2.3));
  
  
  chunk.write(chunk::OpCode::OpConstant as u8, 0);
  chunk.write(a, 0);
  chunk.write(chunk::OpCode::OpConstant as u8, 0);
  chunk.write(b, 0);
  
  chunk.write(chunk::OpCode::OpConstant as u8, 0);
  chunk.write(c, 0);
  
  chunk.write(chunk::OpCode::OpFunction as u8, 0);

  chunk.write(chunk::OpCode::OpReturn as u8, 0);

  chunk.disassemble("test chunk");
  
  let mut vm = vm::VM::new(&chunk);
  vm.interpret();
}

fn repl() {
  loop {
    print!("> ");
    stdout().flush().unwrap();
    
    let mut line = String::new();
    let _ = stdin().read_line(&mut line);
    if line.chars().nth(0).is_none() { 
      println!();
      break 
    }

    let mut chars = line.chars();
    chars.next_back();
    run(chars.as_str().to_owned());
  }
}

fn file() {
  let filename = env::args().nth(1).unwrap();
  let file = fs::read_to_string(filename.clone()).expect(&format!("Unable to read file: {}", filename));
  run(file);
}

fn run(to_run: String) {
}