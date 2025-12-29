# Module

Code in all crates + programs is organized into modules.

Every crate has at least one module, the root module. The root module is the crate itself.

And crate might have some additional submodules. 

## Visibility

By default, all items are private.

Use `pub` to make items public.

## Visibility Rules

1. Items in a module are private by default.
2. Items in a crate are public by default.
3. Items in a parent module can access items in child modules.
4. Items in a child module can't access items in parent modules.

## Example

To use external create / mod

```rust
fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(1..=10);
    println!("Random number: {}", random_number);
}
```

Or we can use `use` keyword, and list out the functions we want to use

```rust
use rand::thread_rng;
use rand::random;
use rand::rngs::OsRng;

fn main() {
    let rng = thread_rng();
    let rand_number = random();
    let rand_u64 = OsRng.next_u64();
}
```

Or:

```rust
use rand::{thread_rng, random, rngs::OsRng};

fn main() {
    let rng = thread_rng();
    let rand_number = random();
    let rand_u64 = OsRng.next_u64();
}
```

To use internal create, we need to use `use` keyword. And `crate` to refer to the root module.

```rust
mod my_module {
    pub fn my_function() {
        println!("Hello, world!");
    }
}
```

```rust
use crate::my_module::my_function;

fn main() {
    my_function();
}
```

## Three ways to create modules

1. Inline modules - Create a mod in an existing file
2. File-based modules - Create a module in a new single file in the same folder
3. Folder-based modules - Create a module in a new folder

### Inline modules

```rust
mod catalog {
    pub fn my_function() {
        println!("Hello, world!");
    }
}

fn main() {
    let catalog = catalog::my_function();
}
```

### File-based modules

File `src/content.rs`
```rust
pub fn my_function() {
    println!("Hello, world!");
}
```

File `src/main.rs`
```rust
// mod content;

use crate::content::my_function;

fn main() {
    my_function();
}
```

### Folder-based modules

File `src/content/content.rs`
```rust
pub fn my_function() {
    println!("Hello, world!");
}
```

File `src/content/catalog.rs`
```rust
pub fn my_another_function() {
    println!("Hello, world from here!");
}
```

File `src/content/mod.rs`
```rust
mod content;
mod catalog;
```

File `src/main.rs`
```rust
mod content;

fn main() {
    content::catalog::my_another_function();
}
```

## Rules

- Every file and folder makes its own separate module
- You can't do deeply nested imports
- You have to chain imports