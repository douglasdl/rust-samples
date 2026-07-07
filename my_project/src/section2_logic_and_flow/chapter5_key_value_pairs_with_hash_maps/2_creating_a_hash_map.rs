/*
Creating a Hash Map



Now that you understand what hash maps are, let's learn how to create one. To use hash maps in Rust, you first need to bring the HashMap type into scope with a use statement at the top of your file:

use std::collections::HashMap;
Once you've imported HashMap, you can create a new, empty hash map using the HashMap::new() function:

let mut scores = HashMap::new();
Notice that we declare the hash map as mut because we'll likely want to add data to it later. When you create an empty hash map this way, Rust doesn't know what types the keys and values will be until you start inserting data.

You can also specify the types explicitly if you prefer:

let mut scores: HashMap<String, i32> = HashMap::new();
This creates a hash map that will store String keys (like student names) and i32 values (like test scores). The empty hash map is now ready to store your key-value pairs.
*/ 