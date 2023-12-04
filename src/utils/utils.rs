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
        format!("Tiempo de ejecución: {} horas, {} minutos, {} segundos y {} milisegundos", hours, minutes, secs, millis)
    } else if minutes > 0 {
        format!("Tiempo de ejecución: {} minutos, {} segundos y {} milisegundos", minutes, secs, millis)
    } else if secs > 0 {
        format!("Tiempo de ejecución: {} segundos y {} milisegundos", secs, millis)
    } else if millis > 0 {
        format!("Tiempo de ejecución: {} milisegundos y {} nanosegundos", millis, nanos)
    } else {
        format!("Tiempo de ejecución: {} nanosegundos", nanos)
    };

    println!("{}", message);
}