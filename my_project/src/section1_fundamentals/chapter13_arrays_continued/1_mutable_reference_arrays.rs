fn process_array(arr: &mut [i32]) {
	arr[0] = 3;
}

fn main() {
  let mut numbers = [1, 2, 3, 4, 5];
  
  // Call process_array
  process_array(&mut numbers);
  
  for i in 0..numbers.len() {
    print!("{} ", numbers[i]);
  }
}