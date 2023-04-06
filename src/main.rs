use std::io::{self, BufRead};

fn main() {
    // Read a string from stdin
    let stdin = io::stdin();
    let input_string = stdin.lock().lines().next().unwrap().unwrap();

    // Reverse the string
    let reversed_string = input_string.chars().rev().collect::<String>();

    // Print the reversed string
    println!("{}", reversed_string);
}
