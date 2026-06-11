fn main() {
  // Fix this code to make it work
  let number = 42;
  {
      let mut number = number;
      println!("number is {}", number);
      number = 100;  // This will cause an error
      println!("number is {}", number);
  }
  println!("number is {}", number);
}