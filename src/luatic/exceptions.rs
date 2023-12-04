pub struct Location {
  row: i64,
  col: i64
}

pub enum Exception {
  ParsingException{msg: String, loc: Location},
  CodeGenException{msg: String, loc: Location},
}
