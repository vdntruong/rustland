# Rust types

## Number

Number is a type that can be used to represent a value

Number can be signed or unsigned

### types and ranges

| Type  | Range                                                                               |
| ----- | ----------------------------------------------------------------------------------- |
| u8    | 0 to 255                                                                            |
| u16   | 0 to 65535                                                                          |
| u32   | 0 to 4294967295                                                                     |
| u64   | 0 to 18446744073709551615                                                           |
| u128  | 0 to 340282366920938463463374607431768211455                                        |
| i8    | -128 to 127                                                                         |
| i16   | -32768 to 32767                                                                     |
| i32   | -2147483648 to 2147483647                                                           |
| i64   | -9223372036854775808 to 9223372036854775807                                         |
| i128  | -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727 |
| isize | depends on the architecture of the system                                           |
| usize | depends on the architecture of the system                                           |
| f32   | 1.175494351e-38 to 3.4028235e+38                                                    |
| f64   | 2.2250738585072014e-308 to 1.7976931348623157e+308                                  |

### operations

- addition
- subtraction
- multiplication
- division
- remainder

## Array

Array is a fixed-size sequence of elements of the same type

```rust
let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
let values = ["2", "3", "4", "5"];
```

## Vector

Vector is a resizable array, or slice in Go

```rust
let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
let values = vec!["2", "3", "4", "5"];
```

## Enum

Enum in Rust is a little bit different from other languages. It is a type that can be used to represent a value that can be one of a set of possible values. It is a way to define a type that can be one of a set of possible values.

Does each thing you're modeling have the same methods? If so, you should use enums.
Does each thing have some same, but also some different methods? If so, you should use structs.

```rust
// Enum without data
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

// Enum with data
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
}
```

## Option

In Rust, we don't have null, or nil, or undefined. We have Option.

Option is a type that can be used to represent a value that can be one of two possible values: Some or None.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Option has two variants: Some and None.

### Unwrap

Unwrap is a method that returns the value inside the Option.

```rust
let item = catalog.items.get(1);
println!("{:#?}", item.unwrap());
```

### Expect

Expect is a method that returns the value inside the Option, or panics if the Option is None.

```rust
let item = catalog.items.get(1);
println!("{:#?}", item.expect("No media found"));
```

### UnwrapOr

UnwrapOr is a method that returns the value inside the Option, or a default value if the Option is None.

```rust
let item = catalog.items.get(1);
println!("{:#?}", item.unwrap_or(&Media::Placeholder));
```
