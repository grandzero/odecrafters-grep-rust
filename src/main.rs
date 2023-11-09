use std::env;
use std::io;
use std::process;

enum MatchValues {
    Default,
    Digit,
    AlphaNumeric,
    EndOfString,
}

enum MatchError {
    InvalidPattern,
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    match_pattern_recursive(input_line, pattern, pattern)
}

fn match_pattern_recursive(input_line: &str, pattern: &str, full_pattern: &str) -> bool {
    if !pattern.contains("[") {
        //println!("{}", pattern);
        // println!("Entered empty spaced pattern");
        // Get resolved pattern
        if input_line.len() == 0 {
            //println!("Input line exhausted");
            return false;
        }
        let clear_pattern = pattern.replace("\\", "");
        let pattern_slice = if pattern.len() >= 2 {
            &pattern[0..2]
        } else {
            pattern
        };
        let resolved_pattern = pattern_resolve(pattern_slice);

        let mut recursive_pattern = full_pattern;
        match resolved_pattern {
            Ok(MatchValues::AlphaNumeric) => {
                if input_line.chars().nth(0).unwrap().is_alphanumeric() {
                    recursive_pattern = &pattern[2..];
                }
            }
            Ok(MatchValues::Digit) => {
                if input_line.chars().nth(0).unwrap().is_digit(10) {
                    recursive_pattern = &pattern[2..];
                }
            }
            Ok(MatchValues::Default) => {
                if input_line.chars().nth(0).unwrap() == clear_pattern.chars().nth(0).unwrap() {
                    recursive_pattern = &pattern[1..];
                }
            }
            Ok(MatchValues::EndOfString) => {
                //println!("Pattern exhausted");
                return true;
            }
            Err(_) => {
                // println!("Invalid pattern");
                return false;
            }
        }
        match_pattern_recursive(&input_line[1..], recursive_pattern, full_pattern)
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

fn pattern_resolve(pattern: &str) -> Result<MatchValues, MatchError> {
    let mut is_match_character = false;
    //println!("patternlen: {}", pattern.len());
    if pattern.len() == 0 {
        return Ok(MatchValues::EndOfString);
    }
    for c in pattern.chars() {
        if is_match_character {
            match c {
                'd' => return Ok(MatchValues::Digit),
                'w' => return Ok(MatchValues::AlphaNumeric),
                '\0' => return Ok(MatchValues::EndOfString),
                _ => return Err(MatchError::InvalidPattern),
            }
        } else {
            match c {
                '\\' => {
                    is_match_character = true;
                }
                _ => {
                    return Ok(MatchValues::Default);
                }
            }
        }
    }
    Err(MatchError::InvalidPattern)
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
