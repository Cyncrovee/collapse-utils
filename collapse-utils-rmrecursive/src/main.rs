use std::{
    env::args,
    fs::remove_dir_all
};

fn main() {
    let args: Vec<_> = args().collect();
    let mut is_confirm = false;
    for arg in &args {
        if arg == "-c" {
            is_confirm = true;
        } else {
            is_confirm = false;
        }
    }

    match is_confirm {
        true => {
            remove_rec(&args);
        } false => {
            remove_rec(&args);
        }
    }
}

fn remove_rec(args: &Vec<String>) {
    for rm_arg in &args[1..args.iter().count()] {
        let rm_result = remove_dir_all(&rm_arg);
        let _ = match rm_result {
            Ok(dir) => dir,
            Err(_) => panic!("Failed to recursively remove directory")
        };
    }
}
