use super::exceptions::Position;

pub trait Scanner<T> {
  fn consume(&mut self);
  fn cur(&self) -> Option<T>;
  fn consume_until(&mut self, predicate: fn(T) -> bool);
  fn look_ahead(&self, step: usize) -> Option<T>;
  fn get_position(&self) -> Position;
}
