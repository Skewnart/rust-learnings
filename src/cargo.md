# Cargo

## Basic commands

```
cargo new
    --bin : binary               : (default)
    --lib : library

cargo init                       : create a new Cargo.toml package
```

```
cargo build ("cargo b")
    --release
```

```
cargo check                      : checks the code / displays warnings & errors
cargo run ("cargo r")            : builds and executes the result
cargo test ("cargo t")           : executes test flags
cargo clean                      : cleans `target` directory (previous builds)
```

```
cargo --version
```

## Configuration :

### Linter

Rust Linter can be configured in VSC :

Parameter named `Rust-analyzer › Check: Command`

- "test" for classical linter (default value)
- "clippy" for improved linter (really good)
