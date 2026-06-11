fn main() {
  // Declare x and initialize it with 5
  let x = 5;
  // Print the value of x
  println!("x is: {}", x);
  {
      // Shadow x with the original x plus 3
      let x = x + 3;
      // Print the value of the shadowed x
      println!("x is: {}", x);
  }
  // Print the value of outer x
  println!("x is: {}", x);
}