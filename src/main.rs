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
mod compiler;
mod stdlib;

use crate::vm::InterpretResult;
use crate::value::Value;
use crate::compiler::Compiler;

fn main() {
  if env::args().len() == 1 {
    repl();
  } else if env::args().len() == 2 {
    file();
  } else {
    panic!("Not that many args please, thank you");
  }
}

fn repl() {
  let mut vm = vm::VM::new();
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
    println!("{}", run(&mut vm, chars.as_str().to_owned()).unwrap_or_default());
  }
}

fn file() {
  let filename = env::args().nth(1).unwrap();
  let file = fs::read_to_string(filename.clone()).expect(&format!("Unable to read file: {}", filename));
  run(&mut vm::VM::new(), file);
}

fn run(vm: &mut vm::VM, to_run: String) -> Option<Value> {
  let mut compiler = Compiler::new(to_run);
  compiler.compile();

  if compiler.had_error {
    return None;
  }
  compiler.chunk.disassemble("test");

  return match vm.interpret(Box::new(compiler.chunk)) {
    InterpretResult::Err(s) => {
      println!("{s}");
      None
    },
    InterpretResult::Ok(s) => Some(s),
    _ => None,
  }
}