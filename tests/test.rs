#[cfg(test)]
mod tests {
  use luatic::compiler::compile;

  #[test]
  fn diff_tests() {
    let mut cwd = std::env::current_dir().unwrap();
    cwd.push("tests");
    cwd.push("ltc");

    let dir = std::fs::read_dir(cwd).unwrap();
    for entry in dir {
      let entry = entry.unwrap();
      let filename = entry.path();
      let ext = filename.extension().unwrap_or_default().to_str().unwrap_or_default();

      if ext.ends_with("ltc") {
        println!("test file {:?}", filename);
        match compile(filename.to_str().unwrap().to_string()) {
          Ok(_) => (),
          Err(why) => println!("{:?} failed: {:?}", filename, why)
        }
      }
    }
  }
}
