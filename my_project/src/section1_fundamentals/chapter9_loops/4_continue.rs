fn main() {
  for i in 1..=20 {
      if(i % 2 == 1) {
         continue; 
      }
      println!("{}", i);
  }
}
