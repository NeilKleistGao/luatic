use std::collections::HashMap;
use crate::binary::{chunk::Chunk, literals::Literal, prototype::{FuncInfo, Prototype}, instructions::Instruction};
use super::{ast::Program, constants::{ConstTable, Constant}};

pub struct Generator {
  program: Program,
  const_table: ConstTable,
  max_regs: u8
}

impl Generator {
  pub fn new(program: Program, const_table: ConstTable) -> Generator {
    Generator { program, const_table, max_regs: 2 }
  }

  fn create_func_info(table: ConstTable, insts: Vec<Instruction>, max_regs: u8) -> FuncInfo {
    let mut constants: HashMap<Literal, u32> = HashMap::new();
    for cs in table {
      match cs {
        Constant::Text { string, translation: _ } => {
          let lit = Literal::Str(string);
          if !constants.contains_key(&lit) {
            constants.insert(lit, u32::try_from(constants.len()).unwrap());
          }
        }
      }
    }

    FuncInfo::new(constants, insts, 2)
  }

  /*
   * @see /docs/lynx.md
   */
  fn build_lynx(&self) -> Vec<Instruction> {
    let table_loc = 0; // * the stack is empty
    let new_table = Instruction::new_table(table_loc, 4, 0);
    vec![
      Instruction::var_arg_prep(),
      new_table.0,
      new_table.1,
      Instruction::ret(0, 2),
      Instruction::ret(0, 1)
    ]
  }

  pub fn generate_chunk(self, source: String) -> Result<Chunk, String> {
    let lynx_insts = self.build_lynx();
    let lynx_table = self.const_table;
    Ok(Chunk::new(1, Prototype::new(source, Generator::create_func_info(lynx_table, lynx_insts, self.max_regs))))
  }
}
