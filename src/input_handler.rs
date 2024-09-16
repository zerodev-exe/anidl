use colored::*;
use std::io;

/// Initializes the input handler by reading a string from the user.
pub fn init_input() -> String {
    let input = read_input("Enter the name of the anime you wish to download :");
    trim(input)
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt.green());
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

/// Trims whitespace from the input and replaces spaces with "%20".
pub fn trim(input: String) -> String {
    // Trim whitespace from the input
    input.trim().replace(" ", "%20")
}

/// Parses a string input into a usize.
pub fn number_parser() -> Result<usize, Box<dyn std::error::Error + 'static>> {
    let input = read_input("Enter the number of the anime :");
    let my_num: usize = input
        .trim()
        .parse()
        .expect("please give me correct string number!");
    Ok(my_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_spaces() {
        let input1 = String::from("Hello, world!");
        let input2 = String::from("Hello,%20world!");

        assert_eq!(trim(input1), input2);
    }

    #[test]
    fn check_no_spaces() {
        let input = String::from("test");
        assert_eq!(input, trim(input.clone()));
    }
}
