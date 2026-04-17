use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

fn generate_password(min_length: usize, numbers: bool, special_characters: bool) -> String {
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = "0123456789";
    let special = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

    let mut characters = String::from(letters);
    if numbers {
        characters.push_str(digits);
    }
    if special_characters {
        characters.push_str(special);
    }

    let chars: Vec<char> = characters.chars().collect();
    let mut rng = thread_rng();

    let mut pwd = String::new();
    let mut has_number = false;
    let mut has_special = false;

    while pwd.len() < min_length 
        || (numbers && !has_number) 
        || (special_characters && !has_special) {
        
        let new_char = *chars.choose(&mut rng).unwrap();
        pwd.push(new_char);

        if numbers && digits.contains(new_char) {
            has_number = true;
        }
        if special_characters && special.contains(new_char) {
            has_special = true;
        }
    }

    pwd
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let min_length: usize = read_input("Enter the minimum length: ")
        .parse()
        .unwrap_or(8);

    let has_number = read_input("Do you want to have numbers (Y/N) ").to_lowercase() == "y";
    let has_special = read_input("Do you want to have Special Characters (Y/N) ").to_lowercase() == "y";

    let pwd = generate_password(min_length, has_number, has_special);
    println!("The generated password is:\n{}", pwd);
}