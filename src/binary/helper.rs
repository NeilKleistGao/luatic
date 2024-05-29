/*
 * (eeeeexxx)
 *   if eeeee == 0 then xxx
 *   else (1xxx) * 2^(eeeee - 1)
 */
pub fn int2fb(x: u8) -> u32 {
  if x < 8 { x.into() }
  else {
    let mut x32: u32 = x.into();
    let mut exp = 0;
    while x32 & 1 == 0 {
      exp += 1;
      x32 >>= 1;
    }

    ((exp + 1) << 3) | (x32 - 8)
  }
}
