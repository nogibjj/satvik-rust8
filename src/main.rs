use std::io::{self, BufRead};

fn main() {
    // Read a list of numbers from stdin
    let stdin = io::stdin();
    let mut numbers = Vec::new();
    let mut ii = 0;
    for line in stdin.lock().lines() {
        if let Ok(num) = line.unwrap().trim().parse::<f64>() {
            numbers.push(num);
        }
        ii += 1;
        if ii > 5{
            break;
        }
    }

    // Calculate the mean of the numbers
    let mean = numbers.iter().sum::<f64>() / numbers.len() as f64;

    // Calculate the standard deviation of the numbers
    let variance = numbers.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (numbers.len() - 1) as f64;
    let std_dev = variance.sqrt();

    // Print the results
    println!("Mean: {}", mean);
    println!("Standard deviation: {}", std_dev);
}
