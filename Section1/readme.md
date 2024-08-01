# Executing the first Rust code

## Using rustc 


```rs
fn main() {
    println!("Hello World!");
}
```

`rustc hello_world.rs` this will create executable binary file called `hello_world`
execute using `./hello_world`

cargo vs rustc

- Use cargo for managing projects, dependencies, building, running, testing, and generating documentation. Cargo is essential for larger, multi-file projects and provides a streamlined workflow.
- Use rustc for compiling simple, single-file Rust programs or when you need to quickly experiment with Rust code. Rustc provides direct control over the compilation process but lacks the project management features of Cargo.

## Using cargo

1. create new project using cargo
cargo new day1_rust


cargo run: Compiles and runs your Rust program in a development environment.
cargo build: Compiles your Rust program into an executable without running it.
cargo build --release: Compiles your Rust program into an optimized, release-ready executable.

cargo run / cargo build will create debug folder and inside this it will have executable binary
cargo build -release: will create release folder and inside this it will have executable binary

can run above builds using 
./target/debug/day1_rust 
./target/release/day1_rust

