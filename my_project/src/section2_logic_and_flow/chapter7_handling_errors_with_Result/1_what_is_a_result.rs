/*
What is a 'Result'?



While Option handles the absence of values, Rust provides another enum for a different but equally important scenario: operations that can fail. Meet Result<T, E> - Rust's primary tool for handling recoverable errors.

The Result enum has two variants that represent the outcome of an operation:

enum Result<T, E> {
    Ok(T),    // Success - contains the successful result
    Err(E),   // Failure - contains error information
}
When an operation succeeds, it returns Ok(value) containing the successful result. When it fails, it returns Err(error) containing information about what went wrong. This explicit handling of success and failure cases makes Rust programs more reliable and predictable.

You'll encounter Result everywhere in Rust - from file operations that might fail due to missing files, to network requests that could timeout, to parsing operations that might receive invalid input. Unlike languages that use exceptions, Rust forces you to acknowledge that operations can fail and handle both outcomes explicitly.
*/ 