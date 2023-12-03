use std::io;

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Fibonacci";
}

pub fn run() {
    println!("Ingresa cuantos números de fibonacci deseas ver.");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Falla al leer la linea");

    let n: usize = n
        .trim()
        .parse()
        .expect("Se ingréso un valor no numérico");


    let mut fib = vec![];

    if n >= 1 { fib.push(0); }

    if n >= 2 { fib.push(1); }

    for i in 2..n {
        fib.push(fib[i - 1] + fib[i - 2]);
    }

    print!("Los primeros {} números de fibonacci son: \n", n);
    println!("{:?}", fib);

    utils::show_press_enter();
}