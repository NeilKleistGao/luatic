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

impl Prototype {
  pub fn empty(source: String) -> Prototype { // TODO: check values
    Prototype {
      source: source,
      line_defined: 0,
      last_line_defined: 0,
      num_params: 0,
      is_vararg: 0,
      max_stack_size: 8,
      code: Vec::new(),
      constants: Vec::new(),
      up_values: Vec::new(),
      proto: Vec::new(),
      line_info: Vec::new(),
      abs_line_info: Vec::new(),
      local_var: Vec::new(),
      up_value_names: Vec::new()
    }
  }
}

pub fn read_prototy() -> Result<Prototype, String> {
  Err("not implemented".to_string())
}

pub fn prototype_to_binary(proto: Prototype) -> Result<Vec<u8>, String> {
  let mut res: Vec<u8> = Vec::new();
  Ok(res)
}
