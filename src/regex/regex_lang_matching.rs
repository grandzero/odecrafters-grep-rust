/**
 * Language Matching
 * ALP => [a-z]
 * DIG => [0-9]
 * DF => ALP | DIG
 * DF => DF + Char
 * DF => DF + Digit
 * STRT => "^"
 * END => "$"
 * SBD => "["
 * SBC => "]"
 * M => DF
 * M => STRT + DF
 * M => DF + END
 * E => SBD + M + SBC
 * GlobalRegex => M | E
 */
#[derive(Debug, Clone)]
pub enum TokenizedRegex {
    StartOfString,
    EndOfString,
    StartBracket,
    EndBracket,
    Alphanumeric,
    ZeroOrMoreTimes(char),
    OneOrMoreTimes(char),
    Digit,
    Char(char),
    DF(Vec<TokenizedRegex>),
    M(Vec<TokenizedRegex>),
    E(Vec<TokenizedRegex>),
    ZeroByte,
}
#[derive(Debug)]
pub enum ErrorTypes {
    NotDF,
    NotM,
    NotE,
}

fn check_zero_bytes(pattern: &str) -> Result<TokenizedRegex, TokenizedRegex> {
    let check: &[u8] = &[115];

    //println!("patternlen: {:?}", pattern.len());
    if pattern.len() == 0
        || (pattern.as_bytes()[0] == check[0] && pattern.chars().nth(0).unwrap() != 's')
    {
        return Ok(TokenizedRegex::ZeroByte);
    }
    Err(TokenizedRegex::ZeroByte)
}

fn zero_one_times_or_more_automata(pattern: &str) -> Result<TokenizedRegex, ErrorTypes> {
    let value = pattern.chars().next().ok_or(ErrorTypes::NotDF)?;
    if value == '?' {
        return Ok(TokenizedRegex::ZeroOrMoreTimes(value));
    } else if value == '+' {
        return Ok(TokenizedRegex::OneOrMoreTimes(value));
    } else {
        return Err(ErrorTypes::NotDF);
    }
}

fn df_tokenizer(
    pattern: &str,
    result: &mut Vec<TokenizedRegex>,
) -> Result<TokenizedRegex, ErrorTypes> {
    // let mut result: Vec<TokenizedRegex> = Vec::new();
    if check_zero_bytes(pattern).is_ok() {
        result.push(TokenizedRegex::ZeroByte);
        return Ok(TokenizedRegex::ZeroByte);
    } else {
        let value = pattern.chars().next().unwrap();
        match df_tokenizer(&pattern[1..], result) {
            Ok(_) => {
                if value.is_alphanumeric() || value == ' ' {
                    result.insert(0, TokenizedRegex::Char(value));
                } else if value == '\\' {
                    if pattern.len() <= 1 {
                        return Err(ErrorTypes::NotDF);
                    } else {
                        match result.get(0) {
                            Some(val) => {
                                if let TokenizedRegex::Char(_) = val {
                                    result.remove(0);
                                }
                            }
                            None => {}
                        }
                        let second_value = pattern.chars().nth(1).unwrap();
                        match second_value {
                            'd' => {
                                result.insert(0, TokenizedRegex::Digit);
                                return Ok(TokenizedRegex::Digit);
                            }
                            'w' => {
                                return {
                                    result.insert(0, TokenizedRegex::Alphanumeric);
                                    Ok(TokenizedRegex::Alphanumeric)
                                }
                            }
                            _ => return Err(ErrorTypes::NotDF),
                        }
                    }
                } else {
                    match zero_one_times_or_more_automata(&pattern[1..]) {
                        Ok(val) => {
                            result.insert(0, val.clone());
                            return Ok(val);
                        }
                        Err(e) => return Err(e),
                    }
                }
                return Ok(TokenizedRegex::Char(value));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

fn m_tokenizer(pattern: &str) -> Result<TokenizedRegex, ErrorTypes> {
    let mut tokenized_vector: Vec<TokenizedRegex> = Vec::new();
    let mut result: Vec<TokenizedRegex> = Vec::new();
    // Case 1: M => DF
    let splitted_pattern;
    let first_val = pattern.chars().next().unwrap();
    let last_val = pattern.chars().last().unwrap();
    if first_val == '^' {
        splitted_pattern = &pattern[1..];
        result.push(TokenizedRegex::StartOfString);
    } else if last_val == '$' {
        splitted_pattern = &pattern[..pattern.len() - 1];
    } else {
        splitted_pattern = pattern;
    }
    if df_tokenizer(splitted_pattern, &mut tokenized_vector).is_ok() {
        result.push(TokenizedRegex::DF(tokenized_vector));
        if last_val == '$' {
            result.push(TokenizedRegex::EndOfString);
        }
    } else {
        return Err(ErrorTypes::NotM);
    }

    Ok(TokenizedRegex::M(result))
}

fn e_tokenizer(pattern: &str) -> Result<TokenizedRegex, ErrorTypes> {
    let mut result: Vec<TokenizedRegex> = Vec::new();
    // Case 1: M => DF
    let splitted_pattern;
    let is_bracket: bool;
    let first_val = pattern.chars().next().unwrap();
    let last_val = pattern.chars().last().unwrap();
    if pattern.contains("[") && !pattern.contains("]")
        || pattern.contains("]") && !pattern.contains("[")
    {
        return Err(ErrorTypes::NotE);
    }
    if first_val == '[' && last_val == ']' {
        is_bracket = true;
        splitted_pattern = &pattern[1..pattern.len() - 1];
    } else {
        return Err(ErrorTypes::NotE);
    }
    let m_output = m_tokenizer(splitted_pattern);
    match m_output {
        Ok(val) => {
            result.push(val);
            if is_bracket {
                result.insert(0, TokenizedRegex::StartBracket);
                result.push(TokenizedRegex::EndBracket);
            }
        }
        Err(_) => {
            return Err(ErrorTypes::NotE);
        }
    }

    Ok(TokenizedRegex::E(result))
}

pub fn r_tokenizer(pattern: &str) -> Result<TokenizedRegex, ErrorTypes> {
    match e_tokenizer(pattern) {
        Ok(val) => {
            return Ok(val);
        }
        Err(_) => {
            return m_tokenizer(pattern);
        }
    }
}
