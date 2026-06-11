fn main() {
  let a: i32 = 11;
  let b: i32 = 11;
  
  // Don't change below this line
  let mut c: i32 = 0;
  if a >= b && !(b < 10) {
      c = 2;
  }
  
  c += 1;
  println!("c = {}", c);
}