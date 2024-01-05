pub fn write_variant(mut n: i64) -> Result<Vec<u8>, String> {
  let mut res: Vec<u8> = Vec::new();
  let mut prefix: u8 = 0b10000000;
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
  Ok(res)
}
