use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        writeln!(io::stderr(), "Usage: cccat <file1> [file2 ...]").unwrap();
        std::process::exit(1);
    }

    for file_name in &args[1..] {
        if file_name == "-" {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input).expect("Failed to read from standard input");
            print!("{}", input);
        } else {
            let mut file = File::open(file_name).unwrap_or_else(|e| {
                writeln!(io::stderr(), "Error opening file: {}", e).unwrap();
                std::process::exit(1);
            });

            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap_or_else(|e| {
                writeln!(io::stderr(), "Error reading file: {}", e).unwrap();
                std::process::exit(1);
            });
            print!("{}", contents);
        }
    }
}