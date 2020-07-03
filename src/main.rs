use cargo_toolchain::get_toolchain;
use std::{env, process};

const HELP_MESSAGE: &str = "
Prints the currently active rustup toolchain

USAGE:
    cargo toolchain [OPTIONS]

OPTIONS:
    -d, --directory\tprint the default toolchain for the directory rather than the currently active one
    -h, --help\tprint this message and exit
";

fn main() {
    let mut directory = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-d" => directory = true,
            "--directory" => directory = true,
            "-h" => return help(),
            "--help" => return help(),
            "toolchain" => continue, // if called from cargo there's an extra argument at the start
            _ => {
                eprintln!("\nUnrecognized argument '{}'", arg);
                help();
                process::exit(1);
            }
        }
    }

    match get_toolchain(directory) {
        Ok(toolchain) => println!("{}", toolchain),
        Err(reason) => {
            eprintln!("error: {}", reason);
            process::exit(1);
        }
    }
}

fn help() {
    eprintln!("{}", HELP_MESSAGE);
}
