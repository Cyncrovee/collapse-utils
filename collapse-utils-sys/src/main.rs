use clap::{Arg, ArgAction, command};
use std::env::consts::{
    ARCH, DLL_EXTENSION, DLL_PREFIX, DLL_SUFFIX, EXE_EXTENSION, EXE_SUFFIX, FAMILY, OS,
};

fn main() {
    let matches = command!()
        .about("Shows information about the system.")
        .arg(
            Arg::new("arch")
                .short('a')
                .long("architecture")
                .help("Print the CPU architecture")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("os")
                .short('o')
                .long("operating-system")
                .help("Print the operating system")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("family")
                .short('f')
                .long("family")
                .help("Print the OS family")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ex-extension")
                .short('e')
                .long("ex-extension")
                .help("Print the executable extension")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ex-suffix")
                .short('E')
                .long("ex-suffix")
                .help("Print the executable suffix")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lib-extension")
                .short('l')
                .long("lib-extension")
                .help("Print the library extension")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lib-suffix")
                .short('s')
                .long("lib-suffix")
                .help("Print the library suffix")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lib-prefix")
                .short('p')
                .long("lib-prefix")
                .help("Print the library prefix")
                .action(ArgAction::SetTrue),
        )
        .get_matches();
    match matches.get_flag("arch") {
        true => println!("{}", ARCH),
        false => {
            // Pass
        }
    }
    match matches.get_flag("os") {
        true => println!("{}", OS),
        false => {
            // Pass
        }
    }
    match matches.get_flag("family") {
        true => println!("{}", FAMILY),
        false => {
            // Pass
        }
    }
    match matches.get_flag("ex-extension") {
        true => println!("{}", EXE_EXTENSION),
        false => {
            // Pass
        }
    }
    match matches.get_flag("ex-suffix") {
        true => println!("{}", EXE_SUFFIX),
        false => {
            // Pass
        }
    }
    match matches.get_flag("lib-extension") {
        true => println!("{}", DLL_EXTENSION),
        false => {
            // Pass
        }
    }
    match matches.get_flag("lib-suffix") {
        true => println!("{}", DLL_SUFFIX),
        false => {
            // Pass
        }
    }
    match matches.get_flag("lib-prefix") {
        true => println!("{}", DLL_PREFIX),
        false => {
            // Pass
        }
    }
}
