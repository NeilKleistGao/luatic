use super::prototype::Prototype;

static magic_number: [u8; 4] = [27, 76, 117, 97]; // TODO: u32?
static version_number: u8 = 84;
static format_number: u8 = 0;
static luac_data: [u8; 6] = [25, 147, 13, 10, 26, 10]; // TODO: u16 + u32?
static instruction_size: u8 = 4;
static integer_size: u8 = 8;
static number_size: u8 = 8;
static luac_int: u64 = 0x5678;
static luac_number: f64 = 370.5;

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
      signature: magic_number,
      version: version_number,
      format: format_number,
      luac_data: luac_data,
      instruction_size: instruction_size,
      integer_size: integer_size,
      number_size: number_size,
      luac_int: luac_int,
      luac_number: luac_number
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

pub fn read_and_check_chunk() -> Result<Chunk, String> {
  Err("not implemented".to_string())
}

pub fn chunk_to_binary(chunk: Chunk) -> Result<Vec<u8>, String> {
  Err("not implemented".to_string())
}
