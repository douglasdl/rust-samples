use std::io;

// TODO: Define the Book struct here with fields: title, author, year, and pages
struct Book {
    title: String, 
    author: String, 
    year: i32, 
    pages: i32
}

fn main() {
    // Read book title
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read line");
    let title = title.trim().to_string();
    
    // Read author name
    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Failed to read line");
    let author = author.trim().to_string();
    
    // Read publication year
    let mut year_input = String::new();
    io::stdin().read_line(&mut year_input).expect("Failed to read line");
    let year: i32 = year_input.trim().parse().expect("Failed to parse year");
    
    // Read number of pages
    let mut pages_input = String::new();
    io::stdin().read_line(&mut pages_input).expect("Failed to read line");
    let pages: i32 = pages_input.trim().parse().expect("Failed to parse pages");
    
    // TODO: Create an instance of the Book struct using the input values
    let book = Book {
        title: title, 
        author: author, 
        year: year, 
        pages: pages
    };

    // TODO: Print the book information in the required format
    println!("Book: {}", book.title);
    println!("Author: {}", book.author);
    println!("Year: {}", book.year);
    println!("Pages: {}", book.pages);
}