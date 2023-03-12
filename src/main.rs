#![allow(dead_code, unused_imports, unused_variables)]
use caesar_cipher_encryption_and_decryption::decrypt;
use caesar_cipher_encryption_and_decryption::encrypt;
use std::io;
use std::io::Write;
fn main() {
    menu();
}

fn encryptor() {
    let mut message = String::new();
    print!("Input the message you want to encrypt: ");
    io::stdout().flush().expect("Error");
    io::stdin()
        .read_line(&mut message)
        .expect("Could not readline");
    let mut shift = String::new();
    print!("Input shift(-27< shift < 27): ");
    io::stdout().flush().expect("Error");
    io::stdin()
        .read_line(&mut shift)
        .expect("Could not readline");
    let message = String::from(message.trim());
    let shift: i32 = shift.trim().parse().unwrap_or(29);
    println!(
        "({}) is the encryption of ({}) with shift({}).",
        encrypt(&message, shift),
        message,
        shift
    );
    println!("\nTaking you back to the main menu...");
    menu()
}

fn decryptor() {
    let mut message = String::new();
    print!("Input the message you want to decrypt: ");
    io::stdout().flush().expect("Error");
    io::stdin()
        .read_line(&mut message)
        .expect("Could not readline");
    let mut shift = String::new();
    print!("Input shift(-27< shift < 27) or any other character/number to check all shifts: ");
    io::stdout().flush().expect("Error");
    io::stdin()
        .read_line(&mut shift)
        .expect("Could not readline");
    let message = String::from(message.trim());
    let shift: i32 = shift.trim().parse().unwrap_or(28);
    if shift > -27 && shift < 27 {
        println!(
            "({}) is the decryption of ({}) with shift({}).",
            decrypt(&message, shift),
            message,
            shift
        );
    } else {
        let mut shift = -26;
        while shift < 27 {
            println!(
                "({}) is the decryption of ({}) with shift({}).",
                decrypt(&message, shift),
                message,
                shift
            );
            shift += 1;
        }
    }
    println!("\nTaking you back to the main menu...");
    menu()
}

fn menu() {
    println!(
        "Do you want to 1. Encrypt
               2. Decrypt
               3. Exit"
    );
    print!("Input your choice: ");
    io::stdout().flush().expect("Error");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Could not readline");
    let option: i8 = option.trim().parse().unwrap_or(5);
    match option {
        1 => encryptor(),
        2 => decryptor(),
        3 => {
            println!("\nThanks for using this program.");
            std::process::exit(1)
        }
        _ => {
            println!("\nWrong input, try again.");
            menu();
        }
    }
}
