pub trait Binary {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String>;
}

pub fn write_binary(filename: String, from: impl Binary) -> Result<(), String> {
  let mut data: Vec<u8> = Vec::new();
  let res = from.to_binary(&mut data);
  match res {
    Err(why) => Err(why),
    Ok(_) => {
      let _ = std::fs::write(filename, data);
      Ok(())
    }
  }
}

impl Binary for i64 {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    let mut res: Vec<u8> = Vec::new();
    let mut prefix: u8 = 0b10000000;
    let mut n = *self;
    while n > 0 {
      let t: u8 = (n & 0b01111111).try_into().unwrap();
      res.push(t | prefix);
      prefix = 0;
      n >>= 7;
    }

    if res.len() > 0 {
      res.reverse();
    }
    else {
      res.push(prefix);
    }
    for b in &res {
      to.push(*b);
    }
    Ok(())
  }
}

impl Binary for usize {
  fn to_binary(&self, to: &mut Vec<u8>) -> Result<(), String> {
    let res: Result<i64, _> = (*self).try_into();
    match res {
      Ok(i) => i.to_binary(to),
      Err(why) => Err(why.to_string())
    }
  }
}
