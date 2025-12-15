# Bindings

Bindings are immutable by default

```rust
let numbers = vec![1, 2, 3]; 

// Error! Cannot mutate immutable binding
numbers.push(4); 

// Error! Cannot reassign either!
numbers = vec![1, 2, 3, 4];
```

To mutate a binding, we need to use `mut` keyword

```rust
let mut numbers = vec![1, 2, 3]; 

numbers.push(4); 

numbers = vec![1, 2, 3, 4, 5];
```
