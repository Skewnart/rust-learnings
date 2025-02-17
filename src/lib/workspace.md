# Workspace

Mieux pour séparer les crates `bin` des crates `lib`

## Structure finale

```
├── Cargo.lock
├── Cargo.toml
├── lib_crate
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── bin_crate
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

## Génération

### Cargo.toml (top level)

``` rust
[workspace]

members = [
    "lib_crate",
    "bin_crate"
]
```

### Dossiers

`cargo new bin_crate`<br>
`cargo new lib_crate --lib`

### Cargo.toml (bin_crate level)

``` rust
[dependencies]

lib_crate = { path = "../lib_crate" }
```