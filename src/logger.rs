use colored::*;

pub mod logger {
    pub fn info(input: str) {
        println!("{} {}", colored::Colorize::blue("INFO"))
    }
}