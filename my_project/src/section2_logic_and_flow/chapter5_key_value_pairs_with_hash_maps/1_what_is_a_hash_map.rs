/*
What is a Hash Map?



Imagine you have a physical dictionary where you can quickly look up any word to find its definition. A hash map works in a similar way - it's a data structure that stores information in key-value pairs, allowing you to quickly find a value by providing its associated key.

In Rust, a hash map is represented by the HashMap<K, V> type, where K is the type of the keys and V is the type of the values. For example, you might have a hash map that stores student names as keys and their test scores as values, or country names as keys and their capital cities as values.

// Example concept: student names → scores
// "Alice" → 95
// "Bob" → 87
// "Carol" → 92
The main advantage of hash maps is their speed - they provide very fast lookups, insertions, and deletions. When you need to find a value, you don't have to search through every item like you would with a vector. Instead, the hash map uses the key to quickly locate the exact position where the value is stored.
*/ 