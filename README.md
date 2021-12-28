# The Rust Programming Language

## Getting Started

### Installing Rust on Linux or macOS

```sh
# install script
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# update rust
rustup update

# uninstall rust
rustup update

# local doc
rustup doc
```

### Compiling and Running

```sh
# compile
rustc main.rs

# run the program
./main
```

### Cargo

Cargo is Rust's build system and package manager. It comes installed with Rust
using the [installation script](#Installing-Rust-on-Linux-or-macOS).

```sh
cargo --version
```

#### Creating a new project with Cargo

```sh
cargo new learning_rust
```

`Cargo.toml`
```toml
[package]
name = "learning_rust"
version = "0.1.0"
edition = "2021"
```

If the project already is `git` initiated, cargo will skip git and adding
`.gitignore`.


#### Build and run the Cargo project

```sh
cargo build
```

`cargo build` will create an executable file in _target/debug/learning_rust_

**Run the program**

```sh
./target/debug/learning_rust

# OR

cargo run
```

`cargo run` will compile the code and then run the resulting executable, while
the former will just print `Hello, World!` to the terminal.

`cargo check` will check if the code compile without producing any executables.

## The Rest of the Chapters's Notes...

3. [Common Programming Concepts](./notes/03-common-programming-concepts.md)
  1. Variables and Mutability
