use colored::*;

pub fn info_print(message: &str) {
    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let begining_sting = "[!]".blue();
    println!("{begining_sting} {} {}", current_time.blue(), message.blue());
}

pub fn error_print(message: &str) {
    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let error_string = "[-]".red();
    println!("{error_string} {} {}", current_time.red(), message.red());
}

pub fn debug_print(message: &str) {
    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let debug_string = "[*]".yellow();
    println!("{debug_string} {} {}", current_time.yellow(), message.yellow());
}

pub fn success_print(message: &str) {
    let current_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let success_string = "[+]".green();
    println!("{success_string} {} {}", current_time.green(), message.green());
}
