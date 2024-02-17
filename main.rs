use sha2::{Sha512, Digest};
use std::io::{self, Write};

fn make_sha512() {
    print!("Enter the string you want to sha512: ");
    io::stdout().flush().expect("Failed to flush");

    let mut hasher = Sha512::new();
    let mut user_input_make: String = String::new();
    io::stdin().read_line(&mut user_input_make).expect("Failed to read line");

    hasher.update(user_input_make);
    let result: sha2::digest::generic_array::GenericArray<u8, sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UTerm, sha2::digest::typenum::B1>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>> = hasher.finalize();
    let hex_result: String = result.iter().map(|byte: &u8| format!("{:02x}", byte)).collect::<String>();

    println!("The sha512 result is: {}", hex_result);
}

fn read_sha512() {
    print!("Enter the expected hash: ");
    io::stdout().flush().expect("Failed to flush");

    let mut expected_hash: String = String::new();
    io::stdin().read_line(&mut expected_hash).expect("Failed to read line");

    let expected_hash: String = expected_hash.trim().to_lowercase();

    print!("Enter the input string: ");
    io::stdout().flush().expect("Failed to flush");

    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let user_input_bytes: &[u8] = user_input.trim().as_bytes();

    let mut hasher = Sha512::new();
    hasher.update(user_input_bytes);
    let user_hash: sha2::digest::generic_array::GenericArray<u8, sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UInt<sha2::digest::typenum::UTerm, sha2::digest::typenum::B1>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>, sha2::digest::typenum::B0>> = hasher.finalize();
    let user_hex_hash: String = user_hash.iter().map(|byte: &u8| format!("{:02x}", byte)).collect::<String>();

    if user_hex_hash == expected_hash {
        println!("Hashes match! User input is verified.");
    } else {
        println!("Hashes do not match! User input could not be verified.");
    }
}

fn main() {
    print!("Enter the operation (Read Sha512 / Make Sha512): ");
    io::stdout().flush().expect("Failed to flush");

    let mut user_input_decide: String = String::new();
    io::stdin().read_line(&mut user_input_decide).expect("Failed to read line");
    let trimmed_input: String = user_input_decide.trim().to_lowercase();

    if trimmed_input == "read" {
        read_sha512();
    } else if trimmed_input == "make" {
        make_sha512();
    } else {
        println!("Invalid operation");
    }
}
