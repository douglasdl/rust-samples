/*
Better Print



We learned how to print an array using a for loop:

let numbers = [1, 2, 3, 4, 5];
    
for num in numbers {
    print!("{} ", num);
}
But there is a simpler way using the {:?} sign:

let numbers = [1, 2, 3, 4, 5];
print!("{:?} ", numbers);
// Output: [1, 2, 3, 4, 5]
{:?} is the debug formatter in Rust. It is commonly used to quickly inspect the contents of arrays, structs, and other data types without writing a custom print loop — especially useful during development and debugging.

Another way it to use with precision (useful for floating-point arrays):

let arr = [1.12345, 2.23456, 3.34567];
println!("{:.2?}", arr);
// Output: [1.12, 2.23, 3.35]
It will also work for simple numbers:

let num = 6.1234;
println!("{:.2?}", num);
// Output: 6.12
*/ 