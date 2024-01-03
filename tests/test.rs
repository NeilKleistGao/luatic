#[cfg(test)]
mod tests {
  use luatic::compiler::{compile, CompileOption};
  use luatic::binary::literals::{Literal};
  #[test]
  fn diff_tests() {
    match std::env::current_dir() {
      Err(why) => panic!("{:?}", why),
      Ok(cwd) => {
        let mut path = cwd;
        path.push("tests");
        path.push("ltc");
        match std::fs::read_dir(path) {
          Err(why) => panic!("{:?}", why),
          Ok(dir) =>
            for entry in dir {
              match entry {
                Err(why) => panic!("{:?}", why),
                Ok(entry) => {
                  let filename = entry.path();
                  let ext = filename.extension().unwrap_or_default().to_str().unwrap_or_default();
                  if ext.ends_with("ltc") {
                    println!("test file {:?}", filename);
                    let _ = compile(CompileOption::new(filename.to_str().unwrap().to_string()));
                    // TODO: check errors
                  }
                }
              }
            }
        }
      }
    }
  }
}
