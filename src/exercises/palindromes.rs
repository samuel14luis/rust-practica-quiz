use std::io;

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Palindromos";
}

pub fn run() {
    println!("TODO: Palindromos");

    utils::show_press_enter();
}