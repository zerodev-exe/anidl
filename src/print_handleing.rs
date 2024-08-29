use colored::*;

/// Prints an error message with a timestamp.
pub fn error_print(message: &str) {
    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let error_string = "[-]".red();
    println!("{error_string} {} {}", current_time.red(), message.red());
}

/// Prints a debug message with a timestamp.
pub fn debug_print(message: &str) {
    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let debug_string = "[*]".yellow();
    println!(
        "{debug_string} {} {}",
        current_time.yellow(),
        message.yellow()
    );
}

/// Prints a success message with a timestamp.
pub fn success_print(message: &str) {
    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let success_string = "[+]".green();
    println!(
        "{success_string} {} {}",
        current_time.green(),
        message.green()
    );
}

/// Prints the list of anime names with their corresponding numbers.
pub fn print_anime_list(anime_name: &[String]) {
    for (index, name) in anime_name.iter().enumerate() {
        let num = format!("[{}]", index + 1);
        println!("{} - {}", num.red(), name);
    }
}
