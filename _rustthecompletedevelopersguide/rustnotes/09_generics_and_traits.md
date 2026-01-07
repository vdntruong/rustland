# Generics & Traits

## Reference

- Dynamically Sized Types - https://doc.rust-lang.org/reference/dynamically-sized-types.html
  - Slices - https://doc.rust-lang.org/reference/types/slice.html
  - Trait Objects - https://doc.rust-lang.org/reference/types/trait-object.html
  - str (String slice) - https://doc.rust-lang.org/reference/types/textual.html 
- Traits https://doc.rust-lang.org/rust-by-example/trait.html
- Returning traits with dyn https://doc.rust-lang.org/rust-by-example/trait/dyn.html
- Supertraits https://doc.rust-lang.org/rust-by-example/trait/supertraits.html
- Exotically Sized Types - https://doc.rust-lang.org/nomicon/exotic-sizes.html

Each instance of a pointer to a trait object includes:
- A pointer to the data
- A pointer to the vtable for the data's type

The vtable is an array of function pointers that implement the trait for the type. (Sound like interface in Go?)
