use std::vec::Vec;
use super::literals::Literal;

struct AbsLineInfo {
  pc: i64,
  line: i64
}

struct UpValue {
  in_stack: u8,
  index: u8,
  kind: u8
}

struct LocalVar {
  var_name: String,
  start_pc: i64,
  end_pc: i64
}

pub struct Prototype {
  source: String,
  line_defined: i64,
  last_line_defined: i64,
  num_params: u8,
  is_vararg: u8,
  max_stack_size: u8,
  code: Vec<u32>,
  constants: Vec<Literal>,
  up_values: Vec<UpValue>,
  proto: Vec<Prototype>,
  line_info: Vec<u8>,
  abs_line_info: Vec<AbsLineInfo>,
  local_var: Vec<LocalVar>,
  up_value_names: Vec<String>
}

pub fn read_prototy() -> Result<Prototype, String> {
  Err("not implemented".to_string())
}
