use std::collections::HashMap;
use std::vec::Vec;
use crate::luatic::ast::FuncInfo;
use crate::to_binary;

use super::literals::Literal;
use super::binary::Binary;
use super::instructions::Instruction;

struct AbsLineInfo {
  pc: i64,
  line: i64
}

impl Binary for AbsLineInfo {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    Ok(()) // TODO
  }
}

struct UpValue {
  in_stack: u8,
  index: u8,
  kind: u8
}

impl UpValue {
  pub fn empty() -> UpValue {
    UpValue { in_stack: 1, index: 0, kind: 0 }
  }
}

impl Binary for UpValue {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    to.push(self.in_stack);
    to.push(self.index);
    to.push(self.kind);
    Ok(())
  }
}

struct LocalVar {
  var_name: String,
  start_pc: i64,
  end_pc: i64
}

impl Binary for LocalVar {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    Ok(()) // TODO
  }
}

pub struct Prototype {
  source: String,
  line_defined: i64,
  last_line_defined: i64,
  num_params: u8,
  is_vararg: u8,
  max_stack_size: u8,
  code: Vec<Instruction>,
  constants: Vec<Literal>,
  up_values: Vec<UpValue>,
  proto: Vec<Prototype>,
  line_info: Vec<u8>,
  abs_line_info: Vec<AbsLineInfo>,
  local_var: Vec<LocalVar>,
  up_value_names: Vec<String>
}

impl Prototype {
  pub fn new(source: String, info: FuncInfo) -> Prototype {
    Prototype {
      source: String::from("@") + &source, // TODO: ignore if it is sub prototype
      line_defined: 0,
      last_line_defined: 0,
      num_params: 0,
      is_vararg: 1,
      max_stack_size: 2,
      code: vec![
        Instruction::var_arg_prep(),
        Instruction::ret(),
      ],
      constants: write_constant_table(info.constants),
      up_values: vec![UpValue::empty()],
      proto: Vec::new(),
      line_info: vec![1, 0],
      abs_line_info: Vec::new(),
      local_var: Vec::new(),
      up_value_names: vec!["_ENV".to_string()]
    }
  }
}

impl Binary for Prototype {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    to_binary!(self.source, to);
    to_binary!(self.line_defined, to);
    to_binary!(self.last_line_defined, to);

    to.push(self.num_params);
    to.push(self.is_vararg);
    to.push(self.max_stack_size);

    to_binary!(self.code, to);
    to_binary!(self.constants, to);
    to_binary!(self.up_values, to);
    to_binary!(self.proto, to);
    to_binary!(self.line_info, to);
    to_binary!(self.abs_line_info, to);
    to_binary!(self.local_var, to);
    to_binary!(self.up_value_names, to);

    Ok(())
  }
}

fn write_constant_table(table: HashMap<Literal, u32>) -> Vec<Literal> {
  let mut temp: Vec<(&Literal, &u32)> = table.iter().collect();
  temp.sort_by(|x, y| x.1.cmp(y.1));
  temp.iter().map(|x| x.0.clone()).collect()
}
