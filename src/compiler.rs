use crate::binary::chunk;
use crate::binary::prototype::Prototype;

use super::luatic::tokenizer::{tokenize};
use super::luatic::exceptions::Exception;
use super::binary::chunk::*;

pub struct CompileOption {
  filename: String,
  output: String
}

impl CompileOption {
  pub fn new(filename: String) -> CompileOption {
    let output_name = filename.replace(".ltc", ".luac");
    CompileOption { filename: filename, output: output_name }
  }
}

fn format_errors(errors: &Vec<Exception>) -> String {
  "".to_string() // TODO
}

fn write_binary(filename: String, chunk: Chunk) -> Result<(), String> {
  let data = chunk_to_binary(chunk);
  match data {
    Err(why) => Err(why),
    Ok(data) => {
      let _ = std::fs::write(filename, data);
      Ok(())
    }
  }
}

pub fn compile(option: CompileOption) -> Result<(), String> {
  match std::fs::read_to_string(&option.filename) {
    Err(why) => Err(why.to_string()),
    Ok(code) => {
      let tokens = match tokenize(code).get_result() {
        Ok(res) => res,
        Err(msgs) => {
          let err = format_errors(msgs);
          return Err(err);
        }
      };

      let chunk = Chunk::new(1, Prototype::empty(option.filename));
      write_binary(option.output, chunk)
    }
  }
}

pub enum Interpretable {
  File(String),
  Code(String),
  ASTNode() // TODO: 
}

// pub fn interpret(content: Interpretable) -> Variant<'static> {
//   Variant::Nil
// }
