use std::env::{
    args,
    consts::{
        ARCH,
        OS,
        FAMILY,
        EXE_EXTENSION,
        EXE_SUFFIX,
        DLL_EXTENSION,
        DLL_SUFFIX,
        DLL_PREFIX
    }
};

fn main() {
    let args: Vec<_> = args().collect();
    match args[1].as_str() {
        "-arch" => {
            println!("{}", ARCH)
        }
        "-os" => {
            println!("{}", OS)
        }
        "-family" => {
            println!("{}", FAMILY)
        }
        "-ext" => {
            println!("{}", EXE_EXTENSION)
        }
        "-exs" => {
            println!("{}", EXE_SUFFIX)
        }
        "-libt" => {
            println!("{}", DLL_EXTENSION)
        }
        "-libs" => {
            println!("{}", DLL_SUFFIX)
        }
        "-libp" => {
            println!("{}", DLL_PREFIX)
        }
        &_ => {
            //
        }
    }
}
