use std::ffi::OsStr;
use std::path::*;

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
