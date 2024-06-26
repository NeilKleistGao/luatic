use num_enum::TryFromPrimitive;

use super::binary::Binary;
use super::helper::*;

#[derive(TryFromPrimitive)]
#[repr(u8)]
enum InstructionIndex {
  Move = 0,
  LoadI,
  LoadF,
  LoadK,
  LoadKx,
  LoadFalse, 
  FalseSkip, 
  LoadTrue,
  LoadNil,
  GetUpValue,
  SetUpValue,
  GetTableUp,
  GetTable,
  GetI,
  GetField,
  SetTableUp,
  SetTable,
  SetI,
  SetField,
  NewTable,
  ISelf,
  AddI,
  AddK,
  SubK,
  MulK,
  ModK,
  PowK,
  DivK,
  IDivK,
  BitAndK,
  BitOrK,
  BitXorK,
  ShiftRightI,
  ShiftLeftI,
  Add,
  Sub,
  Mul,
  Mod,
  Pow,
  Div,
  IDiv,
  BitAnd,
  BitOr,
  BitXor,
  ShiftLeft,
  ShiftRight,
  MMBin,
  MMBinI,
  MMBinK,
  UNM,
  BitNot,
  Not,
  Len,
  Concat,
  Close,
  TBC,
  Jump,
  Equal,
  LessThan,
  LessEqual,
  EqualK,
  EqualI,
  LessThanI,
  LessEqualI,
  GreaterThanI,
  GreaterEqualI,
  Test,
  TestSet,
  Call,
  TailCall,
  Return,
  Return0,
  Return1,
  ForLoop,
  ForPrepare,
  TForPrepare,
  TForCall,
  TForLoop,
  SetList,
  Closure,
  VarArguments,
  VarArgumentsPrepare,
  ExtraArguments
}

pub enum InstFormat {
  ABC{ a: u8, b: u8, c: u8 },
  ABX{ a: u8, bx: u32 },
  ASBX{ a: u8, sbx: i32 },
  AX{ ax: u32 },
  SJ{ sj: i32 },
  ERR
}

pub struct Instruction(u32);

impl Instruction {
  pub fn op(&self) -> u8 {
    (self.0 & 0x7F).try_into().unwrap()
  }
  fn a(&self) -> u8 {
    ((self.0 >> 7) & 0xFF).try_into().unwrap()
  }
  fn b(&self) -> u8 {
    ((self.0 >> 16) & 0xFF).try_into().unwrap()
  }
  fn c(&self) -> u8 {
    ((self.0 >> 24) & 0xFF).try_into().unwrap()
  }
  fn ax(&self) -> u32 {
    self.0 >> 7
  }
  fn bx(&self) -> u32 {
    self.0 >> 15
  }
  fn sbx(&self) -> i32 {
    let t: i32 = (self.0 >> 15).try_into().unwrap();
    t - 0b10000000000000000
  }
  fn sj(&self) -> i32 {
    let t: i32 = (self.0 >> 7).try_into().unwrap();
    t - 0b1000000000000000000000000
  }

  pub fn unapply(&self) -> InstFormat {
    let op = match InstructionIndex::try_from(self.op()) {
      Ok(inst) => inst,
      Err(_) => panic!("invalid insruction.")
    };
    match op {
      InstructionIndex::LoadI | InstructionIndex::LoadF => InstFormat::ASBX { a: self.a(), sbx: self.sbx() },
      InstructionIndex::Jump => InstFormat::SJ { sj: self.sj() },
      InstructionIndex::LoadK | InstructionIndex::LoadKx
        | InstructionIndex::ForLoop | InstructionIndex::ForPrepare
        | InstructionIndex::TForPrepare | InstructionIndex::TForLoop
        | InstructionIndex::Closure => InstFormat::ABX { a: self.a(), bx: self.bx() },
      InstructionIndex::ExtraArguments => InstFormat::AX { ax: self.ax() },
      _ => InstFormat::ABC { a: self.a(), b: self.b(), c: self.c() }
    }
  }

  // TODO: constuctors for different instructions

  pub fn var_arg_prep() -> Instruction {
    Instruction(81) // TODO
  }

  /*
   * first(A): the first position on the stack
   * len1(B): the number of returned value + 1
  */
  pub fn ret(first: u8, len1: u8) -> Instruction {
    let first_32: u32 = first.into();
    let len1_32: u32 = len1.into();
    // Instruction(0x46u32 | ((first_32 << 7) ) | (len1_32 << 16))
    Instruction(0x46u32 | (first_32 << 8) | (len1_32 << 16))
  }

  pub fn ext_arg() -> Instruction {
    Instruction(82) // TODO: ?
  }

  /*
   * loc(A): where to store the new table
   * tsize(B): initial size of table
   * asize(C): initial size of array
   */
  pub fn new_table(loc: u8, tsize: u8, asize: u8) -> (Instruction, Instruction) { // * must be followed by EXTRAARG
    let loc_32: u32 = loc.into();
    let tsize_32: u32 = int2fb(tsize);
    let asize_32: u32 = int2fb(asize);
    (Instruction(0x13u32 | (loc_32 << 7) | (tsize_32 << 16) | (asize_32 << 24)), Instruction::ext_arg()) // TODO: ext_arg?
  }

  /*
   * loc(A): where the table is on the stack
   * key(B): where the key is in the constant table
   * val(Ck): the value of the field
   * is_k(k): constant/register
   */
  pub fn set_field(loc: u8, key: u8, val: u8, is_k: bool) -> Instruction {
    let loc_32: u32 = loc.into();
    let key_32: u32 = key.into();
    let val_32: u32 = val.into();
    let k_u32: u32 = if is_k { 1 << 15 } else { 0 };
    Instruction(0x12u32 | (loc_32 << 7) | k_u32 | (key_32 << 16) | (val_32 << 24))
  }

  // TODO
  pub fn get_table_up() -> Instruction {
    Instruction(0x0Bu32)
  }

  // TODO
  pub fn set_table_up() -> Instruction {
    Instruction(0x0Fu32)
  }
}

impl Binary for Instruction {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    let mut data = self.0.to_be_bytes();
    data.reverse();
    for b in &data {
      to.push(*b);
    }
    Ok(())
  }
}
