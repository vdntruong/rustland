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

