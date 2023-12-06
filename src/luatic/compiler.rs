use super::tokenizer::{tokenize};

pub struct CompileOption {
  filename: String,
  output: String
}

impl CompileOption {
  pub fn new(filename: String) -> CompileOption {
    CompileOption { filename: filename, output: "".to_string() } // TODO: figure out output path
  }
}

pub fn compile(option: CompileOption) -> Result<(), String> {
  match std::fs::read_to_string(&option.filename) {
    Err(why) => Err(why.to_string()),
    Ok(code) => {
      let tokens = tokenize(code);
      Ok(())
    }
  }
}
