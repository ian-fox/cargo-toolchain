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

## Terminal Prompt

```bash
cargo_prompt() {
  # cargo locate-project will tell us if we're in a rust project (sub)directory or not
  cargo locate-project >/dev/null 2>/dev/null || return

  local TOOLCHAIN=`cargo toolchain`

  if [ "$TOOLCHAIN" == "stable" ]; then
    echo -en " \e[32m(stable)\e[00m"
  elif [ "$TOOLCHAIN" == "beta" ]; then
    echo -en " \e[33m(beta)\e[00m"
  elif [ "$TOOLCHAIN" == "nightly" ]; then
    echo -en " \e[31m(nightly)\e[00m"
  else
    echo -en " \e[35m(${TOOLCHAIN})\e[00m"
  fi
}
export PS1='[\u@\h \W`cargo_prompt`]\$ '
```

## Usage as a Library

[https://docs.rs/cargo-toolchain](https://docs.rs/cargo-toolchain)
