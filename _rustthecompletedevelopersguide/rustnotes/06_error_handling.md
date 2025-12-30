# Error Handling

There are some ways to handle errors in Rust:

- `match`, `if let`, `while let` expressions -> When we're ready to meanly handle the error
- `unwrap` and `expect` methods on the result -> Quick debugging, or if we want to crash on an Err()
- `?` operator to unwrap or propagate the result -> When we don't have any way to handle the error **in the current funciton/scope**, and leave it to the caller to handle it

And `Result` and `Option` enums are used to receive the result of a function.
