pub fn show_press_enter() {
    println!("Presiona ENTER para continuar...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();    
}