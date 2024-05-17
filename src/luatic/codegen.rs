use std::collections::HashMap;
use crate::binary::{chunk::Chunk, literals::Literal, prototype::{FuncInfo, Prototype}};
use super::{ast::Program, constants::{ConstTable, Constant}};

pub struct Generator {
  program: Program,
  const_table: ConstTable
}

impl Generator {
  pub fn new(program: Program, const_table: ConstTable) -> Generator {
    Generator { program, const_table }
  }

  fn create_func_info(table: ConstTable) -> FuncInfo {
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

    FuncInfo { constants }
  }

  pub fn generate_chunk(self, source: String) -> Result<Chunk, String> {
    Ok(Chunk::new(1, Prototype::new(source, Generator::create_func_info(self.const_table))))
  }
}
