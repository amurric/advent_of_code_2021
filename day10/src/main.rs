use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

use crate::LineError::{Corrupted, Incomplete};

enum LineError {
    Corrupted(i64),
    Incomplete(Vec<char>),
}

fn check_line_error(line: &str) -> LineError {
    let mut chunk_stack = Vec::new();

    for character in line.chars() {
        match character {
            '(' => chunk_stack.push(')'),
            '[' => chunk_stack.push(']'),
            '{' => chunk_stack.push('}'),
            '<' => chunk_stack.push('>'),
            ')' | ']' | '}' | '>' => {
                if character != chunk_stack.pop().unwrap() {
                    return match character {
                        ')' => Corrupted(3),
                        ']' => Corrupted(57),
                        '}' => Corrupted(1197),
                        '>' => Corrupted(25137),
                        _ => panic!("Unexpected character."),
                    }
                }
            },
            _ => panic!("Unexpected character."),
        }
    }

    Incomplete(chunk_stack)
}

fn first_part(file_content: String) -> i64 {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut score = 0;

    for line in lines {
        score += match check_line_error(line) {
            Corrupted(line_score) => line_score,
            _ => 0,
        }
    }

    score
}

fn calculate_incomplete_line_score(mut chunk_stack: Vec<char>) -> i64 {
    let mut score = 0;

    while chunk_stack.len() > 0 {
        score *= 5;

        score += match chunk_stack.pop() {
            Some(')') => 1,
            Some(']') => 2,
            Some('}') => 3,
            Some('>') => 4,
            _ => panic!("Unexpected character."),
        }
    }

    score
}

fn second_part(file_content: String) -> i64 {
    let lines: Vec<&str> = file_content.lines().collect();
    let mut scores = Vec::new();

    for line in lines {
        match check_line_error(line) {
            Incomplete(chunk_stack) => scores.push(calculate_incomplete_line_score(chunk_stack)),
            _ => (),
        }
    }

    scores.sort();
    scores[scores.len() / 2]
}

fn parse_arguments() -> Part {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        process::exit(1);
    }

    match args[1].as_str() {
        "P1" => First(String::from(&args[2])),
        "P2" => Second(String::from(&args[2])),
        _=> process::exit(1),
    }
}

fn main() {
    let result = match parse_arguments() {
        First(filename) => first_part(fs::read_to_string(filename).unwrap()),
        Second(filename) => second_part(fs::read_to_string(filename).unwrap()),
    };

    println!("{}", result);
}
