/*
Borrow In Loop



When we write a loop in Rust, we have two main ways to work with our data: we can look at it (using &) or take it. For example consider the following data:

let numbers = vec![1, 2, 3, 4, 5];
let mut sum = 0;
Method 1: Looking at the data (using &):

for number in &numbers {
    sum += number;
}
println!("I can use it here: {:?}", numbers);
Method 2: Taking the data (without &):

for number in numbers {
    sum += number;
}
// println!("Can't use {:?} anymore!", numbers);
// This would cause an error
Think of it like this:

Using & is like looking at a photo album: you can see all the photos, but the album stays intact
Not using & is like taking the photos out: you can use them, but the album will be empty afterward
Now this style:

for num in numbers.iter() {}
Is similar to this:

for num in &numbers {}
And a mutation loop:

for num in numbers.iter_mut() { 
    *num *= 2;
}
Can also be written this way:

for num in &mut numbers { 
    *num *= 2;
}
*/ 