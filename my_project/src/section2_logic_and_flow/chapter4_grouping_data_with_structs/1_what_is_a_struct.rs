/*
What is a Struct?



So far, you've worked with individual pieces of data - single numbers, strings, and collections like vectors. But what happens when you need to group related information together? For example, how would you represent a person with a name, age, and email address?

This is where structs come in. A struct is Rust's way of creating custom data types by bundling related values into a single, organized unit.

Each piece of data inside a struct is called a field. Fields have names and types, making it clear what each piece of information represents.

Example of a struct with different field types:

struct Person {
    name: String,
    age: u32,
    active: bool,
}
Here a Person struct have a name field of type String, an age field of type u32, and an active field of type bool.
*/ 