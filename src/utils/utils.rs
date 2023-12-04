use std::{time::Duration, io};
use rand::Rng;

pub fn show_press_enter() {
    println!("Presiona ENTER para continuar...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();    
}

pub fn show_duration(duration: Duration) {
    let hours = duration.as_secs() / 3600;
    let minutes = (duration.as_secs() % 3600) / 60;
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    let nanos = duration.subsec_nanos();

    let message = if hours > 0 {
        format!("Tiempo de ejecución: {} h, {} m, {} s y {} ms", hours, minutes, secs, millis)
    } else if minutes > 0 {
        format!("Tiempo de ejecución: {} m, {} s y {} ms", minutes, secs, millis)
    } else if secs > 0 {
        format!("Tiempo de ejecución: {} s y {} ms", secs, millis)
    } else if millis > 0 {
        format!("Tiempo de ejecución: {} ms y {} ns", millis, nanos)
    } else {
        format!("Tiempo de ejecución: {} ns", nanos)
    };

    println!("{}", message);
}

pub fn random_i32() -> i32 {
    let mut rng = rand::thread_rng();
    return rng.gen::<i32>();
}

pub fn random_usize() -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen::<usize>();
}

pub fn generate_random_i32_vec(lenght: usize) -> Vec<i32> {
    let mut vec = vec![];

    for _ in 0..lenght {
        vec.push(random_i32());
    }

    return vec;
}

pub fn generate_random_usize_vec(lenght: usize) -> Vec<usize> {
    let mut vec: Vec<usize> = vec![];

    for _ in 0..lenght {
        vec.push(random_usize());
    }

    return vec;
}

pub fn read_usize() -> usize {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Falla al leer la linea");

    let n: usize = n
        .trim()
        .parse()
        .expect("Se ingréso un valor no numérico");

    return n;
}

pub fn read_i32() -> i32 {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Falla al leer la linea");

    let n: i32 = n
        .trim()
        .parse()
        .expect("Se ingréso un valor no numérico");

    return n;
}