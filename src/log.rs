use crate::{RED, WHITE, GREEN, RESET};

pub fn done(message: &String) {
    println!("{GREEN}==>{WHITE} {message}{RESET}");
}

pub fn error(message: &String) {
    eprintln!("{GREEN}==>{WHITE} {message}{RESET}");
}

pub fn init_panic(message: &String, exit: i32) {
    eprintln!();
    eprintln!("{RED}Init panicked!{RESET}");
    eprintln!("{RED}Error: {WHITE}{message}{RESET}");
    std::process::exit(exit);
}