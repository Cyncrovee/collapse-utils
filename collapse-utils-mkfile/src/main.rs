use std::{env::args, fs::File};

fn main() {
    let args: Vec<_> = args().collect();
    for file_arg in &args[1..args.iter().count()] {
        File::create(&file_arg).expect("Failed to create file");
    }
}
