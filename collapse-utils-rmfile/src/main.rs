use std::{env::args, fs::remove_file};

fn main() {
    // "Collect" arguments passed in the CLI
    let args: Vec<_> = args().collect();
    // For each argument passed, delete the file of the same name (if possible)
    for rm_file_arg in &args[1..args.iter().count()] {
        let rm_file_result = remove_file(&rm_file_arg);
        let _ = match rm_file_result {
            Ok(file) => file,
            Err(_) => eprintln!("Failed to remove file"),
        };
    }
}
