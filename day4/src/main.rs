use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn parse_bingo_system(lines: Vec<&str>) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
    let mut numbers = Vec::new();

    for number in lines[0].split(',') {
        numbers.push(number.parse::<i32>().expect("Not a number."));
    }

    let mut boards = Vec::new();
    let mut board = Vec::new();

    for line in lines.iter().skip(2) {
        match *line {
            "" => {
                boards.push(board);
                board = Vec::new();
            }
            _=> {
                let mut line_numbers = Vec::new();

                let mut numbers: Vec<&str> = (*line).split(' ').collect();
                numbers.retain(|&x| x != "");

                for number in numbers {
                    line_numbers.push(number.parse::<i32>().expect("Not a number."));
                }

                board.push(line_numbers);
            }
        }
    }

    (numbers, boards)
}

fn update_boards(number: i32, boards: &mut Vec<Vec<Vec<i32>>>) {
    for i in 0..boards.len() {
        for j in 0..boards[i].len() {
            for k in 0..boards[i][j].len() {
                if boards[i][j][k] == number {
                    boards[i][j][k] = -1;
                }
            }
        }
    }
}

fn check_winner(boards: &Vec<Vec<Vec<i32>>>) -> Option<usize> {
    for (index, board) in boards.iter().enumerate() {
        for h_line in board {
            if h_line.iter().sum::<i32>() == -5 {
                return Some(index)
            }
        }

        for k in 0..board[0].len() {
            let mut sum = 0;
            for j in 0..board.len() {
                sum += board[j][k];
            }
            if sum == -5 {
                return Some(index)
            }
        }
    }

    None
}

fn get_score(number: i32, board: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for j in 0..board.len() {
        for k in 0..board[j].len() {
            if board[j][k] != -1 {
                sum += board[j][k];
            }
        }
    }
    sum * number
}

fn first_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    let (numbers, mut boards) = parse_bingo_system(lines);

    for number in numbers {
        update_boards(number, &mut boards);
        match check_winner(&boards) {
            Some(index) => return get_score(number, &boards[index]),
            None => (),
        }
    }

    panic!("Winner not found.")
}

fn second_part(file_content: String) -> i32 {
    let lines: Vec<&str> = file_content.lines().collect();
    let (numbers, mut boards) = parse_bingo_system(lines);
    let mut last_score: Option<i32> = None;

    for number in numbers {
        update_boards(number, &mut boards);

        while match check_winner(&boards) {
            Some(index) => {
                last_score = Some(get_score(number, &boards[index]));
                boards.remove(index);
                true
            },
            None => false,
        } {}
    }

    last_score.expect("Winner not found.")
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
