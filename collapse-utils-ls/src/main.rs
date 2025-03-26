use std::{
    env::{args, current_dir},
    fs::{DirEntry, read_dir},
    io::Error,
};

fn main() {
    // Set initial variables
    let args: Vec<_> = args().collect();
    let dir = current_dir().unwrap();
    let dir_contents = read_dir(dir).unwrap();
    let mut is_size_shown = false;
    let mut is_hidden_shown = false;

    // Parse CLI args
    for arg in args {
        if arg == "-s" {
            is_size_shown = true;
        }
        if arg == "-a" {
            is_hidden_shown = true;
        }
    }

    // Main print loop
    for item in dir_contents {
        match is_size_shown {
            // Only print the file size if "-s" was passed on the CLI
            true => {
                let file_size = item.as_ref();
                print!("{}", file_size.unwrap().metadata().unwrap().len());
                print!(" ");
            }
            false => {
                // Pass
            }
        }
        match item
            .as_ref()
            .unwrap()
            .file_name()
            .into_string()
            .unwrap()
            .chars()
            .nth(0)
            .unwrap()
        {
            // Only print hidden items if the "-a" flag was passed
            '.' => {
                match is_hidden_shown {
                    true => {
                        print_item(item);
                    }
                    false => {
                        // Pass
                    }
                }
            }
            _ => {
                print_item(item);
            }
        }
    }
}

fn print_item(item: Result<DirEntry, Error>) {
    let file_name = item.as_ref();
    println!("{}", file_name.unwrap().file_name().into_string().unwrap());
}
