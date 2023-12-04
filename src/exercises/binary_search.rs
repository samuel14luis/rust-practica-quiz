use std::time::{Instant, Duration};

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Búsqueda binaria";
}

pub fn run() {
    println!("Ingresa el arreglo de cuantos números deseas generar.");
    let n: usize = utils::read_usize();

    // generar un vector con n números aleatorios
    let numbers: Vec<i32> = utils::generate_random_i32_vec(n);

    println!("Arreglo generado correctamente:");
    println!("{:?}", numbers);

    // leer el número a buscar
    println!("Ingresa el número que deseas buscar en el arreglo generado.");
    let n: i32 = utils::read_i32();

    let start = Instant::now();

    // hacer una búsqueda binaria
    let index = binary_search(numbers, n);

    let duration: Duration = start.elapsed();

    if index == -1 {
        println!("El número {} no se encuentra en el arreglo.", n);
    } else {
        println!("El número {} se encuentra en el índice {} del arreglo.", n, index);
    }

    utils::show_duration(duration);
    utils::show_press_enter();
}

fn binary_search(arr: Vec<i32>, n: i32) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;
    let mut middle: usize;

    while left <= right {
        middle = (left + right) / 2;

        if arr[middle] == n {
            return middle as i32;
        } else if arr[middle] < n {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    return -1;
}