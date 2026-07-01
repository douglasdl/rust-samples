/*
Mutable Iteration



Sometimes you need to do more than just read vector elements - you need to modify them. While the previous lesson showed how to iterate and read values, this lesson teaches you how to change the elements during iteration using mutable references.

To modify elements while iterating, you need to use &mut instead of & in your for loop:

let mut numbers = vec![1, 2, 3, 4];

for number in &mut numbers {
    *number = *number * 2;
}
The &mut numbers creates a mutable reference to the vector, and each iteration gives you a mutable reference to an individual element. The * operator (called the dereference operator) is used to access and modify the actual value that the reference points to.

This pattern is incredibly useful for transforming data in place. Instead of creating a new vector with modified values, you can efficiently update the existing one. The vector must be declared as mut for this to work, since you're changing its contents.

After the loop above, numbers would contain [2, 4, 6, 8] - each original value doubled in place.

Challenge

Easy
Write a function add_ten_to_all that takes a mutable vector numbers and adds 10 to each element in place, then returns the modified vector.
Use mutable iteration with &mut to modify each element directly. Remember to use the dereference operator * to access and modify the actual values.

Parameters:

numbers (Vec<i32>): A mutable vector of integers to modify
Returns: The modified vector with 10 added to each element (Vec<i32>)

*/ 

fn add_ten_to_all(mut numbers: Vec<i32>) -> Vec<i32> {
    // Write code here
    for number in &mut numbers {
        *number = *number + 10;
    }
    numbers
}
