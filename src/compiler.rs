use crate::frontend::parse;
use crate::luatic::ast::{Program, Statement, Expression};
use crate::luatic::constants::{scan, ConstTable, Constant};

use std::fmt::Write;

pub enum Seperator {
  Comma,
  Semicolon,
  Tab
}

// TODO: language config
struct CompileOption {
  filename: String,
  prefix: String,
  output: (String, String, String),
  seperator: Seperator
}

static EXT_NAME: &str = ".ltc";
  
impl CompileOption {
  pub fn new(filename: String, seperator: Seperator) -> CompileOption {
    let binary_name = filename.replace(EXT_NAME, ".luac");
    let trans_name = filename.replace(EXT_NAME, ".csv");
    let map_name = filename.replace(EXT_NAME, ".map");
    let begin = match filename.rfind(std::path::MAIN_SEPARATOR) {
      Some(i) => i + 1,
      _ => 0
    };
    let prefix = filename[begin..].to_string().replace(EXT_NAME, "");
    CompileOption { filename, prefix, output: (binary_name, trans_name, map_name), seperator }
  }
}

fn read_to_string(path: &String) -> Result<String, String> {
  match std::fs::read_to_string(path) {
    Ok(code) => Ok(code),
    Err(err) => Err(err.to_string())  
  }
}

// TODO: allow language list?
fn export_csv(filename: String, prefix: String, seperator: Seperator, language: &String, table: &ConstTable) -> Result<(), String> {
  let sep_char = match seperator {
    Seperator::Comma => ',',
    Seperator::Semicolon => ';',
    Seperator::Tab => '\t'
  };

  let mut res = String::new();
  let _ = writeln!(&mut res, "{}{}{}", "keys", sep_char, language);

  let mut index = 0;
  for cs in table {
    match cs {
      Constant::Text { string, translation } if *translation => {
        let key = prefix.clone() + &index.to_string();
        let _ = writeln!(&mut res, "{}{}{}", key, sep_char, string);
        index += 1;
      }
      _ => ()
    }
  }

  let _ = std::fs::write(filename, res);
  Ok(())
}

pub fn compile(filename: String, seperator: Seperator) -> Result<(), String> {
  let option = CompileOption::new(filename, seperator);
  let code = read_to_string(&option.filename)?;
  let program = parse(code)?;

  let const_table = scan(&program);
  let _ = export_csv(option.output.1, option.prefix, option.seperator, &program.1, &const_table)?;

  Ok(())
}
