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

fn main() {
  let mut compiler = compiler::Compiler::new();  
  compiler.compile();

  if compiler.had_error {
    return;
  }
  compiler.chunk.disassemble("test");

  let mut vm = vm::VM::new(&compiler.chunk);
  match vm.interpret() {
    vm::InterpretResult::RuntimeError(s) => {
      println!("{s}");
      return;
    },
    _ => (),
  }
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