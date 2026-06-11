fn main() {
  let b1: bool = true;
  let b2: bool = true;
  let b3: bool = false;
  
  let b4: bool = b1 && b2 && (!b3);
  println!("b4 = {}", b4);

}