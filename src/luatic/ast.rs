use std::collections::HashMap;

use crate::binary::literals::Literal;

pub trait ExpressionLike {
  
}

pub trait StatementLike {
  
}

impl ExpressionLike for Literal {
  
}

impl StatementLike for Literal {
  
}

pub struct Definition {
  pub name: String
  // TODO: expression
}

impl StatementLike for Definition {
}

pub struct FuncInfo {
  pub constants: HashMap<Literal, u32>
}

impl FuncInfo {
  pub fn new(csts: HashMap<Literal, u32>) -> FuncInfo {
    FuncInfo { constants: csts }
  }
}

pub struct Program {
  pub main: FuncInfo
}

impl Program {
  pub fn new(info: FuncInfo) -> Program {
    Program { main: info }
  }
}
