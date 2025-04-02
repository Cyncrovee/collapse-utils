use std::env::{
    args,
    consts::{ARCH, DLL_EXTENSION, DLL_PREFIX, DLL_SUFFIX, EXE_EXTENSION, EXE_SUFFIX, FAMILY, OS},
};

fn main() {
    let args: Vec<_> = args().collect();
    for arg in args {
        match arg.as_str() {
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
            "-h" | "--help" => {
                help();
            }
            &_ => {
                //
            }
        }
    }
}

fn help() {
    println!("-a OR --arch             : Print the CPU architecture");
    println!("-o OR --operating-system : Print the operating system");
    println!("-f OR --family           : Print the OS family");
    println!("-ext OR --ex-extension   : Print the executable extension");
    println!("-exs OR --ex-suffix      : Print the executable suffix");
    println!("-libt OR --lib-extension : Print the library extension");
    println!("-libs OR --lib-suffix    : Print the library suffix");
    println!("-libp OR --lib-prefix    : Print the library prefix");
    print!("\n");
    println!("-h OR --help : Print this help text");
}
