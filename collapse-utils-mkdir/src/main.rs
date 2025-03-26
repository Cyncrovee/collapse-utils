use std::{
    env::args,
    fs::create_dir_all
};

fn main() {
    let args: Vec<_> = args().collect();
    for dir_arg in &args[1..args.iter().count()] {
        let create_dir_result = create_dir_all(&dir_arg);
        let _ = match create_dir_result {
            Ok(dir) => dir,
            Err(_) => panic!("Failed to create directory")
        };
    }
}
