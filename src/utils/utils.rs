use std::time::Duration;

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