use std::io;

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Números primos";
}

pub fn run() {
    println!("TODO: Números primos");

    utils::show_press_enter();
}