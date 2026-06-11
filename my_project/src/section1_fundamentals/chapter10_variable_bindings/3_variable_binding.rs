/*
Variable Binding



In Rust, when we say let var;, we're creating a "binding." Think of it as reserving a name (var) that will be connected (or "bound") to a value later.

Direct Binding (Most Common):

// Declare AND initialize in one step
let x = 2;
This is what you'll use 90% of the time. It's clean and straightforward.

Separate Declaration and Initialization:

let var;  // Just declaring
let x = 2;
var = x * x;  // Initializing later
Why Have Separate Declaration?
Conditional Initialization:

let number;
if some_condition {
    number = 1;
} else {
    number = 2;
}
Complex Calculations:

let result;
// ... some complex logic ...
result = final_value;
When Working with Scopes:

let outside;
{
    let inside = 42;
    outside = inside * 2;
}
// inside is gone, but outside keeps its value
*/ 