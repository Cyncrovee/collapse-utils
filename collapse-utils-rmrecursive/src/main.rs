use std::{
    env::args,
    fs::remove_dir_all,
    io::stdin
};

fn main() {
    let args: Vec<_> = args().collect();
    let mut is_confirm = false;
    for arg in &args {
        match arg.as_str() {
            "-c" | "--confirm" => is_confirm = true,
            "-h" | "--help" => help(),
            &_ => {
                // Pass
            }
        }
    }
    match is_confirm {
        true => {
            manual_confirmation(&args);
        }
        false => {
            remove_rec(&args);
        }
    }
}

fn remove_rec(args: &Vec<String>) {
    for rm_arg in &args[1..args.iter().count()] {
        let rm_result = remove_dir_all(&rm_arg);
        let _ = match rm_result {
            Ok(_) => {
                println!("Success: {}", &rm_arg);
            }
            Err(_) => eprintln!("Failed: {}", rm_arg),
        };
    }
}

fn manual_confirmation(args: &Vec<String>) {
    print!("Are you sure you want to remove following the directory(ies) and their contents?:");
    for rm_arg in &args[1..args.iter().count()] {
        print!(" ");
        print!("{}", &rm_arg)
    }
    print!("\n");
    println!("(y/n) ");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: Failed to read input");
    input = input.trim().to_string();
    if input == "y".to_string() {
        print!("\n");
        remove_rec(&args);
    } else {
        println!("Operation cancelled");
    }
}

fn help() {
    println!("rmrecursive: Removes a directory and it's contents");
    println!("-c OR --confirm : Ask for user confirmation via a y/n prompt before continuing");
    println!("-h OR --help    : Show this help message");
}
