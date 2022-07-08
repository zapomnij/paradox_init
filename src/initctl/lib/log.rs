use crate::lib::{RED, WHITE, GREEN, RESET};

pub fn done(message: &String) {
    println!("{GREEN}==>{WHITE} {message}{RESET}");
}

pub fn error(message: &String) {
    eprintln!("{RED}==>{WHITE} {message}{RESET}");
}