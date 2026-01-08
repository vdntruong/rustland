# Clippy setting up!

## Prerequisites
```bash
# check rustc version
rustc --version

# check rustup components
rustup component list

# install clippy (if not installed)
rustup component add clippy
```

## Linters

We can use `cargo clippy --list` to list all available linters.

We can use `cargo clippy -- --all` to run all linters.

To define policy for specific linters:

- Allow
```rust
#![allow(clippy::too_many_arguments)]
```

- Warn
```rust
#![warn(clippy::too_many_arguments)]
```

- Deny
```rust
#![deny(clippy::too_many_arguments)]
```

## Usage

```bash
# Check help
cargo clippy --help

# Run clippy
cargo clippy

# Run clippy with auto fix
cargo clippy -- --fix

# Check exit code (for CI/CD)
echo $?
# 0 means no errors
# 101 means lint errors
# 102 means other errors
# ...
```

