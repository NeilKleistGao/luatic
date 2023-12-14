use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Variant<'a> {
  Str(&'a String),
  Int(i64),
  Num(f64),
  Bool(bool),
  Nil
  // TODO: more types
}

pub struct Context<'a> {
  env: HashMap<String, Variant<'a>>
}

impl<'a> Context<'a> {
  pub fn look_up(&self, name: &String) -> Variant<'a> {
    match self.env.get(name) {
      Some(v) => *v,
      _ => Variant::Nil
    }
  }

  pub fn add(&mut self, name: &String, value: Variant<'a>) {
    self.env.insert(name.clone(), value);
  }
}
