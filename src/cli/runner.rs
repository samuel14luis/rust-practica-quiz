use std::thread;

#[path = "../exercises/first_fibonacci.rs"]
mod fibonacci;

pub fn run() {

    // arreglo de items del menú
    let menu_items = vec![
        fibonacci::get_title(),
        "Salir",
    ];

    // arreglo de funciones a ejecutar
    let func_items: Vec<fn()> = vec![
        fibonacci::fibonacci_first_n,
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
        ms_sleep(170);
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