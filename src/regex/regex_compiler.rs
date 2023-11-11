use super::{r_tokenizer, TokenizedRegex};
use std::cmp::PartialEq;
trait CheckEquality {
    fn check_equality(&self, other: char) -> bool;
}

impl PartialEq for TokenizedRegex {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TokenizedRegex::Char(val) => {
                if let TokenizedRegex::Char(other_val) = other {
                    return val == other_val;
                }
            }
            TokenizedRegex::Alphanumeric => {
                if let TokenizedRegex::Alphanumeric = other {
                    return true;
                }
            }
            TokenizedRegex::Digit => {
                if let TokenizedRegex::Digit = other {
                    return true;
                }
            }
            TokenizedRegex::ZeroByte => {
                if let TokenizedRegex::ZeroByte = other {
                    return true;
                }
            }
            TokenizedRegex::StartOfString => {
                if let TokenizedRegex::StartOfString = other {
                    return true;
                }
            }
            TokenizedRegex::EndOfString => {
                if let TokenizedRegex::EndOfString = other {
                    return true;
                }
            }
            _ => return false,
        }
        return false;
    }
}

impl CheckEquality for TokenizedRegex {
    fn check_equality(&self, other: char) -> bool {
        match self {
            TokenizedRegex::Char(val) => {
                return val == &other;
            }
            TokenizedRegex::Alphanumeric => {
                return other.is_alphanumeric();
            }
            TokenizedRegex::Digit => {
                return other.is_digit(10);
            }
            _ => false,
        }
    }
}

fn input_contains(
    input_line: &str,
    tokenized_pattern: &[TokenizedRegex],
    full_tokenized_pattern: &[TokenizedRegex],
) -> bool {
    if tokenized_pattern[0] == TokenizedRegex::ZeroByte {
        return true;
    }
    if input_line.len() == 0 {
        return false;
    }

    let value = input_line.chars().nth(0).unwrap();
    if tokenized_pattern[0].check_equality(value) {
        return input_contains(
            &input_line[1..],
            &tokenized_pattern[1..],
            full_tokenized_pattern,
        );
    } else {
        return input_contains(
            &input_line[1..],
            full_tokenized_pattern,
            full_tokenized_pattern,
        );
    }
}

fn df_compile(input_line: &str, tokenized_pattern: &[TokenizedRegex], criteria: bool) -> bool {
    // M -> DF
    // println!(
    //     "input_line: {:?}, tokenized_pattern : {:?}",
    //     input_line, tokenized_pattern
    // );
    if input_line.len() == 0 {
        return !criteria;
    }
    if tokenized_pattern[0] == TokenizedRegex::ZeroByte {
        return criteria;
    }
    let current_value = input_line.chars().nth(0).unwrap();
    for pat in tokenized_pattern {
        if pat.check_equality(current_value) {
            return criteria;
        }
    }
    return df_compile(&input_line[1..], tokenized_pattern, criteria);
}

fn compile_m(input_line: &str, pattern: &str, tokenized_pattern: Vec<TokenizedRegex>) -> bool {
    // M -> STR + DF
    if tokenized_pattern.starts_with(&[TokenizedRegex::StartOfString]) {
        return input_line.starts_with(&pattern[1..]);
    }
    // M -> DF + END
    else if tokenized_pattern.ends_with(&[TokenizedRegex::EndOfString]) {
        return input_line.ends_with(&pattern[..pattern.len() - 1]);
    } else {
        // M -> DF

        match tokenized_pattern.get(0) {
            Some(TokenizedRegex::DF(val)) => return input_contains(input_line, &val[..], &val[..]),
            _ => return false,
        }
    }
}
fn compile_e(input_line: &str, tokenized_pattern: &[TokenizedRegex]) -> bool {
    if tokenized_pattern[0] == TokenizedRegex::StartOfString {
        return df_compile(input_line, &tokenized_pattern[1..], false);
        // [^abc]
    } else {
        // [abc]
        // [\wbc]
        return df_compile(input_line, &tokenized_pattern[..], true);
    }
}

pub fn compile_regex(input_line: &str, pattern: &str) -> bool {
    let tokenized_pattern = r_tokenizer(pattern);
    // println!("tokenized_pattern: {:?}", tokenized_pattern);
    match tokenized_pattern {
        Ok(TokenizedRegex::M(value)) => return compile_m(input_line, pattern, value),
        Ok(TokenizedRegex::E(value)) => match value.get(1) {
            Some(TokenizedRegex::M(val)) => compile_e(input_line, &val[..]),
            _ => return false,
        },
        Err(_) => {
            println!("Invalid pattern");
            return false;
        }
        Ok(_) => panic!("Invalid tokenized regex type"),
    }
}
