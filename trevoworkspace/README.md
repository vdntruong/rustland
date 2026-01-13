# Workspace

## What is a workspace?

A workspace is a collection of related packages that are built together.

## Reference

- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/reference/workspaces.html#workspaces)

## Command

- cargo build
- cargo test

When we run `cargo build` or `cargo test`, Cargo will build all the packages in the workspace.

We can specify default members that we want to build/test by using the `default-members` field.

## Setting up

### Cargo.toml

```toml
[workspace]
resolver = "3"

members = [
    "myapp01",
    "mylib01"
]
```

- resolver: help Cargo to resolve/locate dependencies/packages

### Add local dependencies

```bash
cargo add mylib01         # the crate is already added to the workspace
cargo add --path mylib01  # if the crate is not in the workspace, we have to specify the path
```
