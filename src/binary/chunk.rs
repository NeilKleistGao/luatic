use super::prototype::{Prototype};
use super::binary::Binary;

static MAGIC_NUMBER: [u8; 4] = [27, 76, 117, 97]; // TODO: u32?
static VERSION_NUMBER: u8 = 84;
static FORMAT_NUMBER: u8 = 0;
static LUAC_DATA: [u8; 6] = [25, 147, 13, 10, 26, 10]; // TODO: u16 + u32?
static INSTRUCTION_SIZE: u8 = 4;
static INTEGER_SIZE: u8 = 8;
static NUMBER_SIZE: u8 = 8;
static LUAC_INT: u64 = 0x5678;
static LUAC_NUMBER: f64 = 370.5;

struct Header {
  signature: [u8; 4],
  version: u8,
  format: u8,
  luac_data: [u8; 6],
  instruction_size: u8,
  integer_size: u8,
  number_size: u8,
  luac_int: u64,
  luac_number: f64
}

impl Header {
  pub fn new() -> Header {
    Header {
      signature: MAGIC_NUMBER,
      version: VERSION_NUMBER,
      format: FORMAT_NUMBER,
      luac_data: LUAC_DATA,
      instruction_size: INSTRUCTION_SIZE,
      integer_size: INTEGER_SIZE,
      number_size: NUMBER_SIZE,
      luac_int: LUAC_INT,
      luac_number: LUAC_NUMBER
    }
  }
}

impl Binary for Header {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    match validate_header(&self) {
      Ok(_) => (),
      Err(why) => return Err(why)
    }
  
    for b in &self.signature {
      to.push(*b);
    }
    to.push(self.version);
    to.push(self.format);
    for b in &self.luac_data {
      to.push(*b);
    }
    to.push(self.instruction_size);
    to.push(self.integer_size);
    to.push(self.number_size);
    let mut luac_int = self.luac_int.to_be_bytes();
    luac_int.reverse(); // TODO: can we make sure it is little endian?
    for b in &luac_int {
      to.push(*b);
    }
    let mut luac_num = self.luac_number.to_be_bytes();
    luac_num.reverse();
    for b in &luac_num {
      to.push(*b);
    }
    Ok(())
  }
}

pub struct Chunk {
  header: Header,
  up_values_size: u8,
  main_proto: Prototype
}

impl Chunk {
  pub fn new(up_value: u8, proto: Prototype) -> Chunk {
    Chunk { header: Header::new(), up_values_size: up_value, main_proto: proto }
  }
}

pub fn read_chunk() -> Result<Chunk, String> {
  Err("not implemented".to_string())
}

fn validate_header(header: &Header) -> Result<(), String> {
  let mut flag = true;
  flag = flag && header.signature == MAGIC_NUMBER;
  flag = flag && header.version == VERSION_NUMBER; // TODO: support multiple versions?
  flag = flag && header.format == FORMAT_NUMBER;
  flag = flag && header.luac_data == LUAC_DATA;
  flag = flag && header.instruction_size == INSTRUCTION_SIZE;
  flag = flag && header.integer_size == INTEGER_SIZE;
  flag = flag && header.number_size == NUMBER_SIZE;
  flag = flag && header.luac_int == LUAC_INT;
  flag = flag && header.luac_number == LUAC_NUMBER;

  if flag { Ok(()) } else { Err("broken chunk".to_string()) }
}

impl Binary for Chunk {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    match self.header.to_binary(to) {
      Ok(()) => (),
      Err(why) => return Err(why)
    }
    to.push(self.up_values_size);
    self.main_proto.to_binary(to)
    
  }
}
