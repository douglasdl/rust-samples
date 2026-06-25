fn describe_number(num: i32) -> String {
  // Write code here
  let description = match num {
      0 => "zero",
      1 => "one",
      2 => "two",
      3 => "three",
      _ => "many",
  };

  return description.to_string();
}