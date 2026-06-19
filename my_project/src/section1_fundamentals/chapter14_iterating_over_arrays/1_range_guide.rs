fn main() {
  let numbers1 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26];
  let numbers2 = [17, 53, 24, 77, 84, 98, 24, 36, 89, 31, 36];
  print!("Array 1: ");
  // Write your code here
  let mut first = true;
  for i in (0..numbers1.len()).rev().step_by(3) {
      if !first {
          print!(", ");
      }
      print!("[{}]", numbers1[i]);
      first = false;
  }
  
  println!();
  print!("Array 2: ");
  // Write your code here
  let mut first = true;
  for i in (0..numbers2.len()).rev() {
      if numbers2[i] % 4 == 0 {
          if !first {
              print!(", ");
          }
          print!("[{}]", numbers2[i]);
          first = false;
      }
  }
}