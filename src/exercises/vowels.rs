use std::io;

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Contar vocales";
}

pub fn run() {
    println!("TODO: Contar vocales");

    utils::show_press_enter();
}