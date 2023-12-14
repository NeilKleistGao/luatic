use super::runtime::*;
use std::marker::PhantomData;

pub trait Expression<'a> {
  fn eval(&self, ctx: &mut Context<'a>) -> Variant<'a>;
}

pub struct Lit<'a> {
  value: Variant<'a>
}

impl<'a> Expression<'a> for Lit<'a> {
  fn eval(&self, ctx: &mut Context<'a>) -> Variant<'a> {
    self.value
  }
}

pub struct Var<'a> {
  name: String,
  _phantom: PhantomData<& 'a str>
}

impl<'a> Expression<'a> for Var<'a> {
  fn eval(&self, ctx: &mut Context<'a>) -> Variant<'a> {
    ctx.look_up(&self.name)
  }
}

pub struct Def<'a> {
  name: String,
  value: dyn Expression<'a>
}

impl<'a> Expression<'a> for Def<'a> {
  fn eval(&self, ctx: &mut Context<'a>) -> Variant<'a> {
    let value = self.value.eval(ctx);
    ctx.add(&self.name, value);
    Variant::Nil
  }
}

pub struct Invoke<'a> {
  list: Vec<Box<dyn Expression<'a>>>
}
