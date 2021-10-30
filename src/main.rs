extern crate clap;
use clap::{App, Arg};
use std::path::Path;

use routines::*;

mod components;
mod instructions;
#[macro_use]
mod macros;
mod routines;
mod types;

fn main() {
    let commands = App::new("CHIP-Oxide")
        .version("0.1.0")
        .author("Erik Hasse <erik.g.hasse@gmail.com>")
        .about("CHIP-8 emulator")
        .arg(
            Arg::with_name("binary")
                .value_name("FILE")
                .help("Binary file to load")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let file_path = Path::new(commands.value_of("binary").unwrap());
    run(file_path);
}
