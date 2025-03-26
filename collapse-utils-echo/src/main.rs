use std::env::args;

fn main() {
    let args: Vec<_> = args().collect();
    for input_args in &args[1..args.iter().count()] {
        println!("{}", &input_args);
    }
}
