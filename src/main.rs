use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match pattern {
        "\\d" => find_digit(input_line).is_some(),
        "\\w" => alpha_numeric(input_line),
        pat => {
            if pat.starts_with("[") && pat.ends_with("]") {
                positive_chars(input_line, pat)
            } else {
                input_line.contains(pat)
            }
        } // _ => input_line.contains(pattern),
    }
    // if pattern.chars().count() == 1 {
    //     return input_line.contains(pattern);
    // } else if pattern == "\\d" {
    //     find_digit(input_line).is_some()
    // } else {
    //     false
    // }
}

fn positive_chars(input_line: &str, pattern: &str) -> bool {
    let clean_pattern = pattern.trim_matches(|c| c == '[' || c == ']');
    for c in clean_pattern.chars() {
        if input_line.contains(c) {
            return true;
        }
    }
    false
}

fn find_digit(input_line: &str) -> Option<usize> {
    for (i, c) in input_line.chars().enumerate() {
        if c.is_digit(10) {
            return Some(i);
        }
    }
    None
}

fn alpha_numeric(input_line: &str) -> bool {
    for c in input_line.chars() {
        if c.is_alphanumeric() {
            return true;
        }
    }
    false
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("{:?}", env::args());
    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    // Uncomment this block to pass the first stage
    if match_pattern(&input_line, &pattern) {
        println!("Exited with 0");
        process::exit(0)
    } else {
        println!("Exited with 1");
        process::exit(1)
    }
}
