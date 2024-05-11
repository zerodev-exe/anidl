use std::io;

pub fn main() -> String {
    // Read input from the user
    let mut input = String::new();
    println!("Enter a string: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    trim(input)
}

fn trim(input: String) -> String {
    // Trim whitespace from the input
    let trimmed_input = input.trim();

    // Replace spaces with "%20"
    let replaced_input = trimmed_input.replace(" ", "%20");

    // Print the result
    return replaced_input;
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
