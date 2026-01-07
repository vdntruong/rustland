# Rustland

Learning Rust fundamentals through hands-on examples and exercises.

## Call out!

- Turbofish syntax (`::<>`)

## 📚 About

This repository contains practical Rust projects and notes as I learn the language. Each project focuses on specific Rust concepts like ownership, borrowing, structs, traits, and the module system.

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

### Running Projects

Navigate to any project directory and run:

```bash
# Navigate to a project
cd bank  # or cd deck

# Run the project
cargo run

# Run without debug info (quiet mode)
cargo run -q

# Build without running
cargo build
```

### Managing Dependencies

```bash
# Add a crate
cargo add <crate-name>

# Remove a crate
cargo remove <crate-name>
```

## 📖 Key Concepts Covered

- **Ownership & Borrowing:** Understanding Rust's memory management model
- **Structs & Implementations:** Creating custom data types and their associated functions
- **References:** Working with `&` (immutable) and `&mut` (mutable) references
- **Traits:** Using derive attributes and implementing traits
- **Vectors:** Dynamic arrays and collection manipulation
- **Modules & Crates:** Organizing code and using external dependencies

## 🎯 Learning Goals

- Master Rust's ownership system
- Understand the borrow checker
- Learn idiomatic Rust patterns
- Build practical applications
- Explore the Rust ecosystem

## 📝 Notes

This is a learning repository, so code may not be production-ready. Each project includes comments explaining the concepts being demonstrated.

## 🔗 Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust Documentation](https://doc.rust-lang.org/)

## 📄 License

This is a personal learning project. Feel free to use it for your own learning purposes.
