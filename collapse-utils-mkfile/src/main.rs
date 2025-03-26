use std::{
    env::args,
    fs::File
};

fn main() {
    let args: Vec<_> = args().collect();
    for file_arg in &args[1..args.iter().count()] {
        let create_file_result = File::create(&file_arg);
        let _ = match create_file_result {
            Ok(file) => file,
            Err(_) => panic!("Failed to create file")
        };
    }
}
