mod sha512;
use std::io::{self, Write};

fn main() {
    print!("Enter the operation (Read Sha512 / Make Sha512): ");
    io::stdout().flush().expect("Failed to flush");

    let mut user_input_decide: String = String::new();
    io::stdin().read_line(&mut user_input_decide).expect("Failed to read line");
    let trimmed_input: String = user_input_decide.trim().to_lowercase();

    if trimmed_input == "read" {
        sha512::read_sha512();
    } else if trimmed_input == "make" {
        sha512::make_sha512();
    } else {
        println!("Invalid operation");
    }
}
