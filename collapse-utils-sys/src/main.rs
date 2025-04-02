use std::env::{
    args,
    consts::{ARCH, DLL_EXTENSION, DLL_PREFIX, DLL_SUFFIX, EXE_EXTENSION, EXE_SUFFIX, FAMILY, OS},
};

fn main() {
    let args: Vec<_> = args().collect();
    match args[1].as_str() {
        "-a" | "--arch" => {
            println!("{}", ARCH)
        }
        "-o" | "--operating-system" => {
            println!("{}", OS)
        }
        "-f" | "--family" => {
            println!("{}", FAMILY)
        }
        "-ext" | "--ex-extension" => {
            println!("{}", EXE_EXTENSION)
        }
        "-exs" | "--ex-suffix" => {
            println!("{}", EXE_SUFFIX)
        }
        "-libt" | "--lib-extension" => {
            println!("{}", DLL_EXTENSION)
        }
        "-libs" | "--lib-suffix" => {
            println!("{}", DLL_SUFFIX)
        }
        "-libp" | "--lib-prefix" => {
            println!("{}", DLL_PREFIX)
        }
        &_ => {
            //
        }
    }
}
