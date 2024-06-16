use colored::*;
use std::io;

pub fn init_input() -> String {
    // Read input from the user
    let mut input = String::new();
    let output = "Enter the name of the anime you wish to download :".green();
    println!("{}", output);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let begin = trim(input);

    return begin;
}

pub fn trim(input: String) -> String {
    // Trim whitespace from the input
    let trimmed_input = input.trim();

    // Replace spaces with "%20"
    let replaced_input = trimmed_input.replace(" ", "%20");

    // Print the result
    return replaced_input;
}

pub fn number_parser() -> usize {
    // Read input from the user
    let mut input = String::new();
    let output = "Enter the number of the anime :".green();
    println!("{}", output);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let my_num: usize = input
        .trim()
        .parse()
        .expect("please give me correct string number!");

    return my_num;
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