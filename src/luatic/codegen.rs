use std::collections::HashMap;
use crate::binary::{chunk::Chunk, literals::Literal, prototype::{FuncInfo, Prototype}, instructions::Instruction};
use super::{ast::Program, constants::{ConstTable, Constant}};

pub struct Generator {
  program: Program,
  const_table: ConstTable,
  max_regs: u8
}

type ConstHash = HashMap<Literal, u32>;

impl Generator {
  pub fn new(program: Program, const_table: ConstTable) -> Generator {
    Generator { program, const_table, max_regs: 2 }
  }

  fn build_hash_table(const_table: &ConstTable) -> ConstHash {
    let mut constants: ConstHash = HashMap::new();
    for cs in const_table {
      match cs {
        Constant::Text { string, translation: _ } => {
          let lit = Literal::Str(string.clone());
          if !constants.contains_key(&lit) {
            constants.insert(lit, u32::try_from(constants.len()).unwrap());
          }
        }
      }
    }

    constants
  }

  fn create_func_info(table: ConstHash, insts: Vec<Instruction>, max_regs: u8) -> FuncInfo {
    // TODO: max_regs
    FuncInfo::new(table, insts, 4)
  }

  fn find_constant(consts: &ConstHash, name: &str) -> u8 {
    let key = Literal::Str(name.to_string());
    match consts.get(&key) {
      Some(id) => (*id) as u8,
      _ => panic!("unexpected constant {}", name)
    }
  }

  fn build_dialogs(&self, consts: &ConstHash) -> Vec<Instruction> {
    let table_loc = 1; // * as long as we do not erase the global table
    let new_table = Instruction::new_table(table_loc, 0, 0);
    let mut res = vec![new_table.0, new_table.1]; // TODO
    res.push(Instruction::set_field(0, Generator::find_constant(consts, "dialogues"), table_loc, false));
    res
  }

  fn build_variables(&self, consts: &ConstHash) -> Vec<Instruction> {
    let table_loc = 1; // * as long as we do not erase the global table
    let new_table = Instruction::new_table(table_loc, 0, 0);
    let mut res = vec![new_table.0, new_table.1]; // TODO
    res.push(Instruction::set_field(0, Generator::find_constant(consts, "variables"), table_loc, false));
    res
  }

  fn build_commands(&self, consts: &ConstHash) -> Vec<Instruction> {
    let table_loc = 1; // * as long as we do not erase the global table
    let new_table = Instruction::new_table(table_loc, 0, 0);
    let mut res = vec![new_table.0, new_table.1]; // TODO
    res.push(Instruction::set_field(0, Generator::find_constant(consts, "code"), table_loc, false));
    res
  }

  /*
   * @see /docs/lynx.md
   */
  fn build_lynx(&self, consts: &ConstHash) -> Vec<Instruction> {
    let table_loc = 0; // * the stack is empty
    let new_table = Instruction::new_table(table_loc, 4, 0);
    let mut res = vec![
      Instruction::var_arg_prep(),
      new_table.0,
      new_table.1
    ];

    let mut dialogs = self.build_dialogs(consts);
    res.append(&mut dialogs);
    let mut variables = self.build_variables(consts);
    res.append(&mut variables);
    let mut commands = self.build_commands(consts);
    res.append(&mut commands);

    res.push(Instruction::ret(0, 2));
    res.push(Instruction::ret(0, 1));

    res
  }

  pub fn generate_chunk(self, source: String) -> Result<Chunk, String> {
    let lynx_table = Generator::build_hash_table(&self.const_table);
    let lynx_insts = self.build_lynx(&lynx_table);
    Ok(Chunk::new(1, Prototype::new(source, Generator::create_func_info(lynx_table, lynx_insts, self.max_regs))))
  }
}
