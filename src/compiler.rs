use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::InputStream;
use crate::frontend::luaticlexer::LuaticLexer;
use crate::frontend::luaticparser::LuaticParser;
use crate::frontend::parse;
use crate::luatic::ast::{Program, Statement, Expression};

struct CompileOption {
  filename: String,
  output: (String, String, String)
}
  
impl CompileOption {
  pub fn new(filename: String) -> CompileOption {
    let binary_name = filename.replace(".ltc", ".luac");
    let trans_name = filename.replace(".ltc", ".csv");
    let map_name = filename.replace(".ltc", ".map");
    CompileOption { filename: filename, output: (binary_name, trans_name, map_name) }
  }
}

fn read_to_string(path: &String) -> Result<String, String> {
  match std::fs::read_to_string(path) {
    Ok(code) => Ok(code),
    Err(err) => Err(err.to_string())  
  }
}

pub fn compile(filename: String) -> Result<(), String> {
  let option = CompileOption::new(filename);
  let code = read_to_string(&option.filename)?;
  let program = parse(code)?;

  Ok(())
}
