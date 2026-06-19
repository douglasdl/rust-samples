fn main() {
  let mut prices = [2.75, 1.50, 5.00, 3.5, 4.1, 2.25, 7.9];

  println!("Original Prices:");
  // TODO: Use enumerate to print each item number and price
  // Expected: "Item 1: $2.99" etc.
  for (index, value) in prices.iter().enumerate() { 
      println!("Item {}: ${:}", index + 1, value);
  }


  println!("\nBundle Deals:");
  // TODO: Use chunks to print pairs of prices and their sums
  // Expected: "Bundle 1: $2.99 + $1.50 = $4.49" etc.
  for (index, chunk) in prices.chunks(2).enumerate() {
      if chunk.len() == 2 {
          println!(
              "Bundle {}: ${:} + ${:} = ${:}",
              index + 1,
              chunk[0],
              chunk[1],
              chunk[0] + chunk[1]
          );
      } else {
          println!(
              "Bundle {}: ${:}",
              index + 1,
              chunk[0]
          );
      }
  }
  

  // TODO: Use iter_mut to apply 10% discount to all prices
  for price in prices.iter_mut() { 
      *price *= 0.9;
  }

  println!("\nPrices after 10% discount:");
  // TODO: Print final prices after discount
  // Expected: "$2.69" etc.
  for (index, value) in prices.iter().enumerate() {
      println!("${:}", value);
  }

}