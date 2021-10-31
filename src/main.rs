extern crate clap;
use clap::{App, Arg};
use std::path::Path;

use routines::*;

mod components;
mod instructions;
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
        .arg(
            Arg::with_name("leading_zeros")
                .long("zeros")
                .short("z")
                .help("Input binary has leading zeros")
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .help("Print program state")
        )
        .get_matches();

    let file_path = Path::new(commands.value_of("binary").unwrap());
    let leading_zeros = commands.is_present("leading_zeros");
    let verbose = commands.is_present("verbose");
    run(file_path, leading_zeros, verbose);
}
