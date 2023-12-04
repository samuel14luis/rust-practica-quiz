use std::time::{Instant, Duration};

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Algortimo de ordenamiento por inserción";
}

pub fn run() {
    println!("Ingresa el arreglo de cuantos números deseas generar.");
    let n: usize = utils::read_usize();

    // generar un vector con n números aleatorios
    let numbers: Vec<i32> = utils::generate_random_i32_vec(n);

    let first_element = numbers[0];
    let middle_element = numbers[n / 2];
    let last_element = numbers[n - 1];

    println!("Arreglo generado correctamente:");
    println!("{:?}", numbers);

    let start = Instant::now();
    
    let sorted_arr = insertion_sort(numbers);

    let duration: Duration = start.elapsed();

    println!("Arreglo ordenado correctamente:");
    println!("{:?}", sorted_arr);
    
    // comparar los elementos del primer arreglo con los del segundo arreglo
    // para verificar que estén ordenados correctamente
    println!("Primer elemento: Inicial {} - Final {}", first_element, sorted_arr[0]);
    println!("Elemento medio: Inicial {} - Final {}", middle_element, sorted_arr[n / 2]);
    println!("Último elemento: Inicial {} - Final {}", last_element, sorted_arr[n - 1]);


    utils::show_duration(duration);
    utils::show_press_enter();
}

fn insertion_sort(arr: Vec<i32>) -> Vec<i32> {
    let len = arr.len();
    let mut aux = arr.to_vec();
    for i in 1..len {
        let mut j = i;
        while j > 0 && aux[j - 1] > aux[j] {
            aux.swap(j - 1, j);
            j -= 1;
        }
    }

    return aux;
}