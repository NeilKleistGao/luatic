use super::prototype::{Prototype, prototype_to_binary};

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

pub fn chunk_to_binary(chunk: Chunk) -> Result<Vec<u8>, String> {
  let mut res: Vec<u8> = Vec::new();
  let header = chunk.header;
  match validate_header(&header) {
    Ok(_) => (),
    Err(why) => return Err(why)
  }

  for b in &header.signature {
    res.push(*b);
  }
  res.push(header.version);
  res.push(header.format);
  for b in &header.luac_data {
    res.push(*b);
  }
  res.push(header.instruction_size);
  res.push(header.integer_size);
  res.push(header.number_size);
  let mut luac_int = header.luac_int.to_be_bytes();
  luac_int.reverse(); // TODO: can we make sure it is little endian?
  for b in &luac_int {
    res.push(*b);
  }
  let mut luac_num = header.luac_number.to_be_bytes();
  luac_num.reverse();
  for b in &luac_num {
    res.push(*b);
  }
  res.push(chunk.up_values_size);
  match prototype_to_binary(chunk.main_proto) {
    Ok(data) => for b in &data {
      res.push(*b);
    }
    Err(why) => return Err(why)
  }

  Ok(res)
}
