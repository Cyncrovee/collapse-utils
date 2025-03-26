use std::{env::args, fs::read_to_string};

fn main() {
    // "Collect" arguments passed in the CLI
    let args: Vec<_> = args().collect();
    // For each argument passed, print the file of the same name (if possible)
    for file in &args[1..args.iter().count()] {
        let print_file_result = read_to_string(&file);
        let _ = match print_file_result {
            Ok(file) => println!("{}", file),
            Err(_) => eprintln!("Failed to print file"),
        };
    }
}
