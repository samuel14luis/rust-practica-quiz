use std::time::{Instant, Duration};

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Búsqueda lineal";
}

pub fn run() {
    println!("Ingresa el arreglo de cuantos números deseas generar.");
    let n: usize = utils::read_usize();

    // generar un vector con n números aleatorios
    let numbers: Vec<usize> = utils::generate_random_usize_vec(n);

    println!("Arreglo generado correctamente:");
    println!("{:?}", numbers);

    // leer el número a buscar
    println!("Ingresa el número que deseas buscar en el arreglo generado.");
    let n: usize = utils::read_usize();

    let start = Instant::now();
    // hacer una búsqueda lineal
    let mut index = -1;
    for i in 0..numbers.len() {
        if numbers[i] == n {
            index = i as i32;
            break;
        }
    }

    let duration: Duration = start.elapsed();

    if index == -1 {
        println!("El número {} no se encuentra en el arreglo.", n);
    } else {
        println!("El número {} se encuentra en el índice {} del arreglo.", n, index);
    }

    utils::show_duration(duration);
    utils::show_press_enter();
}