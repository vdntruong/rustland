# Crate

A crate is a collection of Rust source code files that are compiled together.

> NOTE: Similar to package in Go.

Rust standard library docs at https://doc.rust-lang.org/std/

External crates listing at https://crates.io/, and docs at https://docs.rs/

## Types of Crates

Use a binary (application) template
```bash
cargo init --bin ...
```

Use a library template
```bash
cargo init --lib ...
```

### Binary Crate / Application Crate

A binary crate is a crate that can be compiled into an executable file.

With a `main.rs` file, and `fn main() {}` function, it is a binary crate.

### Library Crate

A library crate is a crate that can be compiled into a shared library file.
