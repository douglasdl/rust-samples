fn get_user_info() -> (String, i32, String) {
  // Create name, age, and email variables here
  let name = String::from("Bob");
  let age = 25;
  let email = String::from("bob@example.com");
  
  // Return the tuple
  (name, age, email)
}

fn main() {
  // Call get_user_info and destructure the returned tuple
  let (name, age, email) = get_user_info();
  
  // Print the values
  println!("Name: {}, Age: {}, Email: {}", name, age, email);
}

