/*
What is an 'Option'?



In programming, you often encounter situations where a value might or might not be present. For example, searching for an item in a list might find the item, or it might not exist at all. Traditional programming languages often use special values like null or -1 to represent "nothing," but this approach can lead to bugs and crashes.

Rust takes a different approach with the Option<T> enum. This is a built-in type that explicitly represents the possibility of absence. Instead of guessing whether a value exists, Rust forces you to handle both cases upfront.

The Option enum has exactly two variants:

Some(value)  // Contains a value
None         // Contains no value
When a function might not return a value, it returns an Option. If there's a value to return, it wraps it in Some. If there's no value, it returns None.

This makes the potential absence explicit in the type system, preventing you from accidentally using a value that doesn't exist.

You've already seen Option in action with hash map's .get() method - it returns Some(&value) when a key exists, or None when it doesn't. This pattern appears throughout Rust, making your code safer and more predictable.
/*