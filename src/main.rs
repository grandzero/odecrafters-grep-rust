use std::env;
use std::io;
use std::process;
#[derive(Debug)]
enum MatchValues {
    Default,
    Digit,
    AlphaNumeric,
    EndOfString,
}
fn match_pattern_recursive(input_line: &str, pattern: &str, is_recursive: bool) -> bool {
    if pattern.contains(" ") {
        println!("Entered empty spaced pattern");
        // Get resolved pattern
        let resolved_pattern = pattern_resolve(pattern);
        //println!("{:?}", resolved_pattern);
        // Iterate through input line and check one by one
        let clear_pattern = pattern.replace("\\", "");
        for (index, character) in input_line.chars().enumerate() {
            match resolved_pattern[index] {
                MatchValues::AlphaNumeric => {
                    if !character.is_alphanumeric() {
                        println!("Alpha numeric failed");
                        if is_recursive {
                            continue;
                        }
                        return false;
                    }
                }
                MatchValues::Digit => {
                    if !character.is_digit(10) {
                        println!("Digit failed");
                        if is_recursive {
                            continue;
                        }
                        return match_pattern_recursive(&input_line[index..], pattern, true);
                    }
                }
                MatchValues::Default => {
                    println!(
                        "{}, {}, {}, {}",
                        index, clear_pattern, input_line, character
                    );
                    if character != clear_pattern.chars().nth(index).unwrap() {
                        println!(
                            "Default failed Character : {}, pattern : {}",
                            character,
                            pattern.chars().nth(index).unwrap()
                        );
                        return false;
                    }
                }
                _ => {
                    println!("End of string");
                    return false;
                }
            }
        }
        true
    } else {
        match pattern {
            "\\d" => find_digit(input_line).is_some(),
            "\\w" => alpha_numeric(input_line),
            pat => {
                if pat.starts_with("[") && pat.ends_with("]") {
                    if pat.contains("^") {
                        return positive_negative_chars(input_line, pat, false);
                    }
                    positive_negative_chars(input_line, pat, true)
                } else {
                    input_line.contains(pat)
                }
            }
        }
    }
}
fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match_pattern_recursive(input_line, pattern, false)
}

fn pattern_resolve(pattern: &str) -> Vec<MatchValues> {
    let mut match_values: Vec<MatchValues> = Vec::new();
    let mut is_match_character = false;
    for c in pattern.chars() {
        if is_match_character {
            is_match_character = false;
            match c {
                'd' => match_values.push(MatchValues::Digit),
                'w' => match_values.push(MatchValues::AlphaNumeric),
                _ => match_values.push(MatchValues::Default),
            }
        } else {
            match c {
                '\\' => is_match_character = true,
                _ => match_values.push(MatchValues::Default),
            }
        }
    }
    match_values.push(MatchValues::EndOfString);
    match_values
}

fn positive_negative_chars(input_line: &str, pattern: &str, is_positive: bool) -> bool {
    let clean_pattern = pattern.trim_matches(|c| c == '[' || c == ']');
    for c in clean_pattern.chars() {
        if input_line.contains(c) {
            return is_positive;
        }
    }
    !is_positive
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
