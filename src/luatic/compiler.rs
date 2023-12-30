use super::tokenizer::{tokenize};
use super::exceptions::Exception;

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

fn write_binary(filename: String, data: Vec<u8>) -> Result<(), String> {
  let _ = std::fs::write(filename, ""); // TODO
  Ok(())
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

      let binary: Vec<u8> = Vec::new(); // TODO: compile
      write_binary(option.output, binary)
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
