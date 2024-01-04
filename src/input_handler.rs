use std::io;

pub fn main() -> String {
    // Read input from the user
    let mut input = String::new();
    println!("Enter a string:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim whitespace from the input
    let trimmed_input = input.trim();

    // Replace spaces with "%20"
    let replaced_input = trimmed_input.replace(" ", "%20");

    // Print the result
    replaced_input
}
