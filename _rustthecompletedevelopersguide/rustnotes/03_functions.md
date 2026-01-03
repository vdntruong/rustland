# Functions

## main function

The `main` function is the entry point of a Rust program.

`fn main() { }`

`fn main() -> i32 { }`

`fn main() -> Result<(), Error> { }`

And it returns an integer exit code. Or `Result<(), Error>` to indicate success or failure. 

## Associated functions

Associated functions are functions that are associated with a type, like static functions in C#.

## Methods

Methods are functions that are associated with a type, like methods in Go.

## Closures

Closures are functions that are defined inline, like lambda functions in Go.

```rust
let add :fn(i32, i32) -> i32 = |x: i32, y: i32| -> i32 { x + y };

let pri :fn() = || println!("Hello, world!");
```

## Generics

Generics are templates for types, like generics in Go.

## Trait

Trait is a collection of methods, like interfaces in Go.

## Macros

Macros are functions that are defined at compile time, like macros in C.

## Related Concepts

### Self

Self is a keyword that represents the type of the current instance, like `this` in C#.

### Function argument types

- Need to store the argument somewhere -> Favor taking ownership (receive a value)
- Need to modify the argument -> Favor taking a mutable reference (receive a reference to a value)
- Need to read/calculate the argument -> Favor taking an immutable/read-only reference (receive a reference to a value)

### Implicit Return

Implicit return is a feature that allows you to return a value from a function without using the `return` keyword.

### ? Operator

The `?` operator is a feature that allows you to return an error from a function without using the `return` keyword.

```rust
fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
}
```

If the `read_to_string` function returns an error (perform unwrap), the `?` operator will return the error from the `main` function.

```rust
fn main() -> Result<(), Error> {
    // let text = fs::read_to_string("logs.txt")?; -> Err("some error")
    return Err("some error");
}
```

If the `read_to_string` function returns a value (perform unwrap), the `?` operator will return the value from the `main` function.

```rust
fn main() -> Result<(), Error> {
    // let text = fs::read_to_string("logs.txt")?; -> Ok(("abc123"))
    let text = "abc123";
}
```
