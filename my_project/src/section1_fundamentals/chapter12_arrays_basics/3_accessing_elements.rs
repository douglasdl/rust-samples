fn values(arr: &[i32]) {
  // Write code here
  for i in 0..arr.len() {
      println!("{}", arr[i]);
  }
}

fn main() {
  let numbers = [10, 20, 30, 40, 50];
  values(&numbers);
}