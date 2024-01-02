pub struct Position {
  row: usize,
  col: usize
}

impl Position {
  pub fn new(r: usize, c: usize) -> Position {
    Position { row: r, col: c }
  }
}

impl std::ops::Add<usize> for Position {
  type Output = Location;
  fn add(self, rhs: usize) -> Self::Output {
    let row = self.row;
    let col = self.col;
    Location::new(self, Position { row: row, col: col + rhs })
  }
}

pub struct Location {
  begin: Position,
  end: Position
}

impl Location {
  pub fn new(b: Position, e: Position) -> Location {
    Location { begin: b, end: e }
  }
}

pub enum Exception {
  ParsingException{msg: String, loc: Location},
  CodeGenException{msg: String, loc: Location},
}