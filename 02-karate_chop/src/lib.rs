// http://codekata.com/kata/kata02-karate-chop/

fn chop_iterative(target: i32, slice: &[i32]) -> isize {
  if 0 == slice.len() {
    return -1;
  }
  let mut start = 0;
  let mut end = slice.len() - 1;
  let mut result: isize = -1;
  let mut done = false;
  while !done {
    let index = (start + end) / 2;
    if slice[index] == target {
      result = index as isize;
      done = true;
    } else if start == end {
      done = true;
    } else if start == index {
      start = end;
    } else if slice[index] < target {
      start = index;
    } else if target < slice[index] {
      end = index;
    }
  }
  result
}

fn chop(target: i32, slice: &[i32]) -> isize {
  chop_iterative(target, slice)
}

fn print_chop(target: i32, slice: &[i32]) -> isize {
  let result = chop(target, slice);
  println!("{} = chop({}, {:?})", result, target, slice);
  result
}

#[no_mangle]
pub extern fn run() {
  print_chop(0, &[]);
  print_chop(4, &[0,1,2,3,4]);
  print_chop(4, &[0,1,2,3,4,5]);
  print_chop(12, &[0,1,2,3,4]);
  print_chop(12, &[0,1,2,3,4,5]);
  print_chop(-12, &[0,1,2,3,4]);
  print_chop(-12, &[0,1,2,3,4,5]);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn run_ok() {
    run();
  }

  #[test]
  fn chop_ok() {
    assert!(-1 == chop(3, &[]));
    assert!(-1 == chop(3, &[1]));
    assert!(0 == chop(1, &[1]));

    assert!(0 == chop(1, &[1, 3, 5]));
    assert!(1 == chop(3, &[1, 3, 5]));
    assert!(2 == chop(5, &[1, 3, 5]));
    assert!(-1 == chop(0, &[1, 3, 5]));
    assert!(-1 == chop(2, &[1, 3, 5]));
    assert!(-1 == chop(4, &[1, 3, 5]));
    assert!(-1 == chop(6, &[1, 3, 5]));

    assert!(0 == chop(1, &[1, 3, 5, 7]));
    assert!(1 == chop(3, &[1, 3, 5, 7]));
    assert!(2 == chop(5, &[1, 3, 5, 7]));
    assert!(3 == chop(7, &[1, 3, 5, 7]));
    assert!(-1 == chop(0, &[1, 3, 5, 7]));
    assert!(-1 == chop(2, &[1, 3, 5, 7]));
    assert!(-1 == chop(4, &[1, 3, 5, 7]));
    assert!(-1 == chop(6, &[1, 3, 5, 7]));
    assert!(-1 == chop(8, &[1, 3, 5, 7]));
  }
}

