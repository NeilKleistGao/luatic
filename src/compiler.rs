use crate::frontend::parse;
use crate::luatic::ast::{Program, Statement, Expression};
use crate::luatic::constants::scan;

pub enum Seperator {
  Comma,
  Semicolon,
  Tab
}

struct CompileOption {
  filename: String,
  output: (String, String, String),
  seperator: Seperator
}
  
impl CompileOption {
  pub fn new(filename: String, seperator: Seperator) -> CompileOption {
    let binary_name = filename.replace(".ltc", ".luac");
    let trans_name = filename.replace(".ltc", ".csv");
    let map_name = filename.replace(".ltc", ".map");
    CompileOption { filename, output: (binary_name, trans_name, map_name), seperator }
  }
}

fn read_to_string(path: &String) -> Result<String, String> {
  match std::fs::read_to_string(path) {
    Ok(code) => Ok(code),
    Err(err) => Err(err.to_string())  
  }
}

// fn export_csv(filename: String, seperator: Seperator, language_list: Vec<String>) -> Result<(), String> {}

pub fn compile(filename: String, seperator: Seperator) -> Result<(), String> {
  let option = CompileOption::new(filename, seperator);
  let code = read_to_string(&option.filename)?;
  let program = parse(code)?;

  let const_table = scan(&program);

  Ok(())
}
