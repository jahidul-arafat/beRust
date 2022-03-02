# Project-1: Create a simple Rust Cargo Project named "hello_cargo"
```shell
# Step-0: Check the cargo and rustc version
> rustc --version
> cargo --version

# Step-1: Create a new cargo project named "hello_cargo"
> cargo new hello_cargo       # create a new project named "hello_wrold"
> cd hello_cargo
> tree

> cat Cargo.toml # Tom’s Obvious, Minimal Languag, which is Cargo's Configuration format
---------------------------
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

---------------------------
> cat src/main.rs
---------------------------
fn main() {
    println!("Hello, world!");
}
---------------------------

# Step-2: Building and running a cargo project in DEV environment
> cargo build       # for dev environment, build as unoptimized version with debuginfo
                    # this will create the following files: Cargo.lock, target/debug/hello_cargo <-- the executable file
> ./target/build/hello_cargo  # run the executable file or try 
> cargo run         # build +run; could be time consuming. so try > cargo check
> cargo check       # dont produce the exectable; must faster and for <DEV> environment


# Step-3: Building and running a cargo project in PRODUCTION environment
> cargo build --release # for PROD environment; optimized and non-debuginfo version.; faster
```
