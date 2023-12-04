use std::io;

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Palindromos";
}

pub fn run() {
    println!("Ingresa el texto a evaluar:");

    let text: String = utils::read_string();

    println!("El texto ingresado es: {}", text.trim());

    let result = is_palindrome(text.trim());

    println!("{}", if result { "Es palindromo." } else { "No es palindromo." });

    utils::show_press_enter();
}

fn is_palindrome(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    let mut j = text.len() - 1;

    while i < j {
        if chars[i] != chars[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    true
}
