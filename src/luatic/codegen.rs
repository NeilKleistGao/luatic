use super::ast::*;
use crate::binary::chunk::Chunk;
use crate::binary::prototype::Prototype;

pub fn gen_chunk(prgm: Program, filename: String) -> Chunk {
  Chunk::new(1, gen_main_proto(prgm.main, filename))
}

fn gen_main_proto(info: FuncInfo, filename: String) -> Prototype {
  Prototype::empty(filename)
}
