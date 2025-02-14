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
cargo check                      : checks the code / displays warnings & errors (alias "cargo c")
cargo run                        : builds and executes the result (alias "cargo r")
cargo test                       : executes test flags (alias "cargo t")
cargo clean                      : cleans `target` directory (previous builds)
```

```
cargo --version
```

## Documentation

```
cargo doc                        : Génère la documentation dans le dossier "target/doc"
cargo doc --open                 : + l'ouvre directement
```

- La documentation se fait au dessus des méthodes avec le "///"

``` rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

Les tests dans la doc sont exécutés avec la commande "test" dans un paragraphe exprès !<br>
Ca permet aux utilisateurs de tester plus facilement les exemples, et aux devs de voir que la doc est toujours en phase

Le "# Examples" est une norme, comme "Panics", "Errors", "Safety"

- La documentation du fichier, module, ou tout ce qui n'est pas pour une fonction se fait avec les "//!"

``` rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
```

## Configuration

### Linter

Rust Linter can be configured in VSC :

Parameter named `Rust-analyzer › Check: Command`

- "test" for classical linter (default value)
- "clippy" for improved linter (really good)

## Publication

- Création de compte sur crates.io
- Récupération du token d'authentification
- `cargo login` avec le token
- Ajout des métadonnées dans le `Cargo.toml` :

``` rust
[package]
name = "package_name"
version = "x.x.x"
edition = "2021"
description = "Description affichée sur crates.io"
license = "MIT"
```

(Les licences sont disponible à [cette adresse](http://spdx.org/licenses/))

- Si la licence n'existe pas, créer un fichier avec cette licence dans le projet et le référencer avec le paramètre `license-file`
- `cargo publish`

Pour flager une version comme "non utilisable", "dépréciée" :<br>
`cargo yank --vers x.x.x`
