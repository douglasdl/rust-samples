fn main() {
  // Initialize variables
  let mut is_sunny = true;
  let mut wind_speed = 5.4;
  let mut temperature = 23;
  let mut solar_panel_output = 9;
  let mut is_cloudy = false;
  
  // The complete logical expression
  let result: bool = is_sunny && ( wind_speed < 10.0 ) && ( solar_panel_output < 15 ) && ( temperature < 20 || ! is_cloudy );
  
  // Don't delete the lines below
  println!("Checking conditions for solar energy production...");
  println!("1. Is it sunny? {}", is_sunny);
  println!("2. Is wind speed safe? {}", (wind_speed < 10.0));
  println!("3. Can panels produce more? {}", (solar_panel_output < 15));
  println!("4. Is temperature good OR no clouds? {}", (temperature > 20 || !is_cloudy));
  println!("\\nFinal result - Good day for solar energy production: {}", result);
}