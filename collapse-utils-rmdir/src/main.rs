use std::{env::args, fs::remove_dir};

fn main() {
    // "Collect" arguments passed in the CLI
    let args: Vec<_> = args().collect();
    // For each argument passed, delete the directory of the same name (if possible)
    for rm_dir_arg in &args[1..args.iter().count()] {
        let rm_dir_result = remove_dir(&rm_dir_arg);
        let _ = match rm_dir_result {
            Ok(dir) => dir,
            Err(_) => eprintln!(
                "Failed to remove directory (Note: this util cannot remove directories with contents- try rmrecursive.)"
            ),
        };
    }
}
