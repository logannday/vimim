use std::fs::File;
use std::io::prelude::*;
use std::env::args;
use sext_editor::{edit_file, read_file};

fn main() {
    let args: Vec<_> = args().collect();

    if args.len() < 2 {
        println!("Usage: provide file name");
        std::process::exit(2);
    }

    let mut file_string = read_file(&args[1]).unwrap_or_else(|err| {
        println!("Failed to read with {}", err);
        std::process::exit(2);
    });
    let mut file = File::create(&args[1]).expect("failed to open");
    edit_file(&mut file, &mut file_string);
}

