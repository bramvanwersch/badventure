use ansi_term::Colour::{Green, Red};

pub fn write_warning(message: &str) {
    println!("{}", Red.paint(message));
}

pub fn write_success(message: &str) {
    println!("{}", Green.paint(message))
}
