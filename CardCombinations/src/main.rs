use std::fmt;
use std::io::{self, Write};

fn main() {
    let total_cards = get_u32(&String::from("Enter Total Number of Cards: "));
    let number_of_distinct_symbols = get_u32(&String::from("Enter number of distinct symbols: "));

    let mut probability: f64 = 1.0;
    let mut denominator = total_cards as f64;
    for i in (1..number_of_distinct_symbols).rev() {
        denominator = denominator - 1.0;
        probability = probability * (i as f64 / denominator);

        println!("# Cards Matching: {}", number_of_distinct_symbols - i + 1);
        println!("Probability: {}%", round_to_decimal(probability * 100.0, 4));
        println!("Odds: 1/{} \n", round_to_decimal(1.0 / probability, 1));
    }

    let trials = get_i32(&String::from("Enter number of trials: "));

    let mut trial = 0;
    for i in 1..trials + 1 {
        trial = trial + get_yn(&format!("Enter trial #{} <Y/N>: ", i));
        print!("Current probability: {}       ", round_to_decimal(trial as f64 / trials as f64 * 100.0,1));
    }
}

// Get a string from the user
fn get_string(prompt: &String) -> String {
    // Space to store the string45
    let mut line = String::new();

    // Show the prompt
    print!("{}", prompt);

    // Make sure the prompt shows before the input
    io::stdout().flush().unwrap();

    // Keep asking for input until it is correct
    loop {
        match io::stdin().read_line(&mut line) {
            Ok(_) => break,
            Err(e) => {
                println!("Input Error! {}\n\n", e);
            }
        }
    }

    line
}

// Get a signed 32 bit integer from the user
fn get_i32(prompt: &String) -> i32 {
    let i: i32;

    loop {
        let line = get_string(&prompt);

        match line.trim().parse::<i32>() {
            Ok(v) => {
                i = v;
                break;
            }
            Err(e) => {
                println!("Integer Input Error! {}\n\n", e);
            }
        }
    }

    i
}

// Get Y or N answer
fn get_yn(prompt: &String) -> i32 {
    let mut result = -1;

    loop {
        let line = get_string(&prompt);

        if line.trim().to_lowercase() == String::from("y") {
            result = 1;
        }
        if line.trim().to_lowercase() == String::from("n") {
            result = 0;
        }
        if result > -1 {break;}
    }

    result
}

// Get a unsigned 32 bit integer from the user
fn get_u32(prompt: &String) -> u32 {
    let i: u32;

    loop {
        let line = get_string(&prompt);

        match line.trim().parse::<u32>() {
            Ok(v) => {
                i = v;
                break;
            }
            Err(e) => {
                println!("Integer Input Error! {}\n\n", e);
            }
        }
    }

    i
}

// Get unsigned 64 bit integer from the user
fn get_u128(prompt: &String) -> u128 {
    let i: u128;

    loop {
        let line = get_string(&prompt);

        match line.trim().parse::<u128>() {
            Ok(v) => {
                i = v;
                break;
            }
            Err(e) => {
                println!("Integer Input Error! {}\n\n", e);
            }
        }
    }

    i
}

// Get float from the user
fn get_f64(prompt: &String) -> f64 {
    let f: f64;

    loop {
        let line = get_string(&prompt);

        match line.trim().parse::<f64>() {
            Ok(v) => {
                f = v;
                break;
            }
            Err(e) => {
                println!("Floating Point Input Error! {}\n\n", e);
            }
        }
    }

    f
}

// Count Combinations
fn count_combinations(n: u128, r: u128) -> u128 {
    if r > n {
        return 0;
    }
    // Optimization: nCr(n, r) == nCr(n, n-r)
    let r = r.min(n - r);

    // Calculate nCr using the formula: (n * (n-1) * ... * (n-r+1)) / r!
    (1..=r).fold(1, |acc, val| acc * (n - val + 1) / val)
}

// Rounding function
fn round_to_decimal(value: f64, places: u32) -> f64 {
    let factor = 10_f64.powi(places as i32);
    (value * factor).round() / factor
}
