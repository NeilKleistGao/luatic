use super::tokenizer::{tokenize};
use super::exceptions::Exception;


pub struct CompileOption {
  filename: String,
  output: String
}

impl CompileOption {
  pub fn new(filename: String) -> CompileOption {
    CompileOption { filename: filename, output: "".to_string() } // TODO: figure out output path
  }
}

fn format_errors(errors: &Vec<Exception>) -> String {
  "".to_string() // TODO
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
      Ok(())
    }
  }
}
