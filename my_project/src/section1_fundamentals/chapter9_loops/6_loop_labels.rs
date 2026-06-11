fn main() {
  'outer: for i in 1..=20 {
      for j in 1..=20 {
          // Your logic here
          if i + j > 10 && (i * j) % 12 == 0 {
              println!("({}, {})", i, j);
              break 'outer;
          }
      }
  }
}