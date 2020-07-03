# cargo-toolchain

`cargo-toolchain` is a utility to get the currently active and default
[rustup toolchains](https://doc.rust-lang.org/stable/book/appendix-07-nightly-rust.html#rustup-and-the-role-of-rust-nightly).

It requires that [rustup](https://rustup.rs/) is installed.

## Usage as a CLI

```shell
cargo install cargo-toolchain

cargo toolchain # prints the currently active cargo toolchain, e.g. 'stable'

cargo toolchain -d # prints the default toolchain for the directory

cargo toolchain -h # print help message
```

## Usage as a Library

[https://docs.rs/cargo-toolchain](https://docs.rs/cargo-toolchain)
