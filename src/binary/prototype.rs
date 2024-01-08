use std::vec::Vec;
use super::literals::Literal;
use super::binary::Binary;

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
      source: String::from("@") + &source,
      line_defined: 0,
      last_line_defined: 0,
      num_params: 0,
      is_vararg: 1,
      max_stack_size: 2,
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

impl Binary for Prototype {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    match (self.source.len() + 1).to_binary(to) {
      Ok(_) => {
        let vs = (self.source).as_bytes();
        for c in vs {
          to.push(*c);
        }
      }
      Err(why) => return Err(why)
    }
    match self.line_defined.to_binary(to) {
      Ok(_) => (),
      Err(why) => return Err(why)
    }
    match self.last_line_defined.to_binary(to) {
      Ok(_) => (),
      Err(why) => return Err(why)
    }
    to.push(self.num_params);
    to.push(self.is_vararg);
    to.push(self.max_stack_size);
    // TODO
    Ok(())
  }
}
