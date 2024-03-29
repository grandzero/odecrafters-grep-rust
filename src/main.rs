mod regex;
use regex::regex_compiler::compile_regex;
use std::env;
use std::io;
use std::process;

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    // let input_line_ci_cd;
    // if input_line.len() == 1 {
    //     input_line_ci_cd = input_line;
    // } else {
    //     input_line_ci_cd = &input_line[..input_line.len() - 1];
    // }
    println!("input_line_ci_cd: {:?}", input_line);
    let val = compile_regex(input_line, pattern);
    val
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
