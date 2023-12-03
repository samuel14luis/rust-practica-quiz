use std::thread;

#[path = "../exercises/first_fibonacci.rs"]
mod fibonacci;

#[path = "../exercises/word_count.rs"]
mod word_count;

#[path = "../exercises/palindromes.rs"]
mod palindromes;

#[path = "../exercises/binary_search.rs"]
mod binary_search;

#[path = "../exercises/vowels.rs"]
mod vowels;

#[path = "../exercises/prime_numbers.rs"]
mod prime_numbers;

#[path = "../exercises/insertion_sort.rs"]
mod insertion_sort;

#[path = "../exercises/linear_search.rs"]
mod linear_search;

pub fn run() {

    // arreglo de items del menú
    let menu_items = vec![
        fibonacci::get_title(),
        word_count::get_title(),
        palindromes::get_title(),
        binary_search::get_title(),
        vowels::get_title(),
        prime_numbers::get_title(),
        insertion_sort::get_title(),
        linear_search::get_title(),
        "Salir",
    ];

    // arreglo de funciones a ejecutar
    let func_items: Vec<fn()> = vec![
        fibonacci::run,
        word_count::run,
        palindromes::run,
        binary_search::run,
        vowels::run,
        prime_numbers::run,
        insertion_sort::run,
        linear_search::run,
        stop,
    ];

    // loop principal (mientras no se seleccione la opción de salir)
    loop {
        print_menu(menu_items.clone());

        let mut option = String::new();

        std::io::stdin()
            .read_line(&mut option)
            .expect("Falla al leer la linea");

        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Opción inválida");
                continue;
            }
        };

        if option > 0 && option <= menu_items.len() as u8 {
            clean_screen();
            one_sec_sleep();
            func_items[(option - 1) as usize]();

            if option == menu_items.len() as u8 {
                break;
            }
        } else {
            println!("Opción inválida");
        }
        
    }
}

fn print_menu(menu_items: Vec<&str>) {
    one_sec_sleep();
    clean_screen();
    println!("Selecciona una opción:");
    for (i, item) in menu_items.iter().enumerate() {
        ms_sleep(if menu_items.len() <= 5 { 150 } else { 90 } );
        println!("{}. {}", i + 1, item);
    }
}

fn stop() {
    clean_screen();
    println!("Cerrando la sesión...");
}

fn one_sec_sleep() {
    thread::sleep(std::time::Duration::from_secs(1));
}

fn ms_sleep(milliseconds: u64) {
    thread::sleep(std::time::Duration::from_millis(milliseconds));
}

fn clean_screen() {
    // Secuencia de escape ANSI para limpiar la pantalla
    print!("\x1B[2J\x1B[1;1H");
}