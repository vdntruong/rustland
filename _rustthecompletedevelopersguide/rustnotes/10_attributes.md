# Attributes

Attributes are metadata that can be attached to items in the code.

## Syntax

```rust
#[attribute]
```

## Examples

Serialize, and Deserialize attributes are used to control how a type is serialized and deserialized.

```rust
#[derive(Serialize, Deserialize)]
struct MyStruct {
    field1: i32,
    field2: String,
}
```
