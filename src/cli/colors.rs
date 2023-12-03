pub fn print_colored(text: &str, color: &str) {
    let color_code = match color {
        "red" => "\x1b[31m",
        "green" => "\x1b[32m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        "magenta" => "\x1b[35m",
        "cyan" => "\x1b[36m",
        "white" => "\x1b[37m",
        _ => "\x1b[0m", // Default color
    };

    println!("{}{}{}", color_code, text, "\x1b[0m"); // Reset to default color after printing
}