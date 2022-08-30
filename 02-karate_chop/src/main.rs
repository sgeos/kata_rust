use lib::run;

fn main() {
  run();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn main_ok() {
    main();
  }
}

