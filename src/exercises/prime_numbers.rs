use std::time::{Instant, Duration};

#[path = "../utils/utils.rs"]
mod utils;

pub fn get_title() -> &'static str {
    return "Números primos";
}

pub fn run() {
    println!("Ingresa cuantos números primos deseas ver.");
    let n: usize = utils::read_usize();


    let start = Instant::now();
    let mut primes = vec![];

    let mut i = 2;
    while primes.len() < n {
        let mut is_prime = true;
        for prime in &primes {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(i);
        }

        i += 1;
    }

    let duration: Duration = start.elapsed();

    print!("Los primeros {} números primos son: \n", n);
    println!("{:?}", primes);

    utils::show_duration(duration);

    utils::show_press_enter();
}