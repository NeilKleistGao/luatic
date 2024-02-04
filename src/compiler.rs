use std::ffi::OsStr;
use std::path::*;

use crate::binary::prototype::Prototype;
use crate::luatic::codegen::gen_chunk;
use crate::luatic::parser::parse;
use crate::luatic::tokenizer::tokenize;
use crate::luatic::exceptions::Exception;
use crate::binary::chunk::*;
use crate::binary::binary::*;

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

pub fn compile(option: CompileOption) -> Result<(), String> {
  match std::fs::read_to_string(&option.filename) {
    Err(why) => Err(why.to_string()),
    Ok(code) => {
      let token_pack = tokenize(code);
      let tokens = match token_pack.get_result() {
        Ok(res) => res,
        Err(msgs) => {
          let err = format_errors(msgs);
          return Err(err);
        }
      };
      let program = parse(tokens);
      let filename = Path::new(&option.filename).file_name().unwrap_or(OsStr::new(""));
      let chunk = gen_chunk(program, filename.to_str().unwrap().to_string());
      match write_binary(option.output, chunk) {
        Ok(_) => Ok(()),
        Err(why) => Err(why.to_string())
      }
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
