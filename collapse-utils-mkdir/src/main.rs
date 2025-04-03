use std::{env::args, fs::create_dir_all};

fn main() {
    let args: Vec<_> = args().collect();
    for dir_arg in &args[1..args.iter().count()] {
        create_dir_all(&dir_arg).expect("Failed to create directory");
    }
}
