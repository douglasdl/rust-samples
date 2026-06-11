fn main() {
  // println! is a macro that prints a line to the console
  println!("Hello, world!");

  /* print! is a macro that prints a string to the console 
  without a newline */
  print!("Hello, ");
  print!("world!");

  println!();

  println!("Hello World!\nThis line was broken up!");

  let name = "Douglas";
  println!("Hello, {}!", name);

  let year = 1987;
  println!("{} was born in {}", name, year);

  println!("{} is {} years old", name, 2026 - year);

  let my_num = 5;         // integer
  let my_double = 5.99;   // float
  let my_letter = 'D';    // character
  let my_bool = true;     // boolean
  let my_text = "Hello";  // string

  println!("my_num: {}", my_num);
  println!("my_double: {}", my_double);
  println!("my_letter: {}", my_letter);
  println!("my_bool: {}", my_bool);
  println!("my_text: {}", my_text);

  let month: i32 = 6;
  println!("month: {}", month);

  let price: f64 = 19.99;
  println!("Price is: R${:.2}", price);

  let my_num2: i32 = 5;         // integer
  let my_double2: f64 = 5.99;   // float
  let my_letter2: char = 'D';    // character
  let my_bool2: bool = true;     // boolean
  let my_text2: &str = "Hello";  // string
  
  println!("my_num2: {}", my_num2);
  println!("my_double2: {}", my_double2);
  println!("my_letter2: {}", my_letter2);
  println!("my_bool2: {}", my_bool2);
  println!("my_text2: {}", my_text2);

  const BIRTHYEAR: i32 = 1987;
  println!("Birthyear: {}", BIRTHYEAR);

  let (name, age) = ("Douglas", 2026 - BIRTHYEAR);
  println!("Name: {}, Age: {}", name, age);

  if age >= 18 {
    println!("You are an adult");
  } else {
    println!("You are a minor");
  }

  let number = 5;
  if number > 0 {
    println!("Number is positive");
  }

  let day = 4;
  match day {
    1 => println!("Sunday"),
    2 => println!("Monday"),
    3 => println!("Tuesday"),
    4 => println!("Wednesday"),
    5 => println!("Thursday"),
    6 => println!("Friday"),
    7 => println!("Saturday"),
    _ => println!("Invalid day"),
  }

  match day {
    1 | 2 | 3 | 4 | 5 => println!("Weekday"),
    6 | 7 => println!("Weekend"),
    _ => println!("Invalid day"),
  }

  let mut count = 1;

  loop {
    println!("Hello World!");

    if count == 3 {
      break;
    }

    count += 1;
  }

  while count <= 5 {
    println!("Count: {}", count);
    count += 1;
  }

  for i in 1..6 {
    println!("i is: {}", i);
  }

  // Create a function
  fn say_hello() {
    println!("Hello from a function!");
  }

  say_hello(); // Call the function

  fn greet(name: &str) {
    println!("Hello, {}!", name);
  }

  greet("Douglas"); // Call the function

  fn add(a: i32, b: i32) -> i32 {
    return a + b;
  }
  
  let sum = add(3, 4);
  println!("Sum is: {}", sum);

  let fruits = ["apple", "banana", "orange"];
  println!("Last fruit: {}", fruits[2]);
  
  let mut fruits = vec!["apple", "banana"];
  fruits.push("cherry");

  println!("Last fruit: {}", fruits[2]);
}
