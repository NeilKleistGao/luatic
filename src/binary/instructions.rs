use super::binary::Binary;

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
    let op = self.op();
    match op { // TODO: replace number with constants
      1 | 2 => InstFormat::ASBX { a: self.a(), sbx: self.sbx() },
      57 => InstFormat::SJ { sj: self.sj() },
      3 | 4 | 73 | 74 | 75 | 77 | 79 => InstFormat::ABX { a: self.a(), bx: self.bx() },
      82 => InstFormat::AX { ax: self.ax() },
      _ => InstFormat::ABC { a: self.a(), b: self.b(), c: self.c() }
    }
  }

  // TODO: constuctors for different instructions

  pub fn var_arg_prep() -> Instruction {
    Instruction(81) // TODO
  }

  pub fn ret() -> Instruction {
    Instruction(16842822) // TODO
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
