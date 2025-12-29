# Functions

## Associated functions

Associated functions are functions that are associated with a type, like static functions in C#.

## Methods

Methods are functions that are associated with a type, like methods in Go.

## Closures

Closures are functions that are defined inline, like lambda functions in Go.

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
