use std::io;

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Contar vocales";
}

pub fn run() {
    println!("Ingresa el texto a evaluar:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Falla al leer la linea");

    let result = vowels_count(n.trim());

    println!("Hemos encontrado {:?} vocales.", result);

    utils::show_press_enter();
}

fn vowels_count(s: &str) -> usize {
    let mut total = 0;
    let vowels = vec![
        "a","e","i","o","u",
        "A","E","I","O","U",
        "á","é","í","ó","ú",
        "Á","É","Í","Ó","Ú"];
    for c in s.chars() {
        if vowels.contains(&c.to_string().as_str()) {
            total += 1;
        }
    }
    total
}
