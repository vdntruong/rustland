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