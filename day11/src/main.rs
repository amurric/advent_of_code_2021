use std::env;
use std::process;
use std::fs;

use crate::Part::{First, Second};

enum Part {
    First(String),
    Second(String),
}

fn parse_grid(lines: Vec<&str>) -> Vec<Vec<i32>> {
    let mut grid = Vec::new();

    for line in lines {
        let mut grid_line = Vec::new();

        for energy in line.chars() {
            grid_line.push(energy.to_digit(10).unwrap() as i32);
        }

        grid.push(grid_line);
    }

    grid
}

fn increase_grid_cell_energy(grid: &mut Vec<Vec<i32>>, flashes: &mut Vec<(usize, usize)>, x: usize, y: usize) {
    grid[y][x] += 1;
    if grid[y][x] > 9 {
        grid[y][x] = 0;
        flashes.push((x, y));
    }
}

fn increase_energy(grid: &mut Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut flashes = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            increase_grid_cell_energy(grid, &mut flashes, x, y);
        }
    }

    flashes
}

fn increase_adjacent_energy(grid: &mut Vec<Vec<i32>>, flashes: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_flashes = Vec::new();

    for (x, y) in flashes {
        for adj_y in (if y != 0 {y - 1} else {y})..(if y + 1 == grid.len() {y + 1} else {y + 2}) {
            for adj_x in (if x != 0 {x - 1} else {x})..(if x + 1 == grid[y].len() {x + 1} else {x + 2}) {
                if adj_x == x && adj_y == y {continue};
                if grid[adj_y][adj_x] != 0 {
                    increase_grid_cell_energy(grid, &mut new_flashes, adj_x, adj_y);
                }
            }
        }
    }

    new_flashes
}

fn simulate_step(grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut flashes = increase_energy(grid);
    let mut count = flashes.len() as i32;

    while flashes.len() > 0 {
        let new_flashes = increase_adjacent_energy(grid, flashes);
        count += new_flashes.len() as i32;
        flashes = new_flashes;
    }

    count
}

fn first_part(file_content: String) -> i32 {
    let mut grid = parse_grid(file_content.lines().collect());
    let mut flashes = 0;

    for _ in 0..100 {
        flashes += simulate_step(&mut grid);
    }

    flashes
}

fn second_part(file_content: String) -> i32 {
    let mut grid = parse_grid(file_content.lines().collect());
    let mut steps = 0;

    while grid.iter().fold(0, |acc, h_line| acc + h_line.iter().sum::<i32>()) > 0 {
        simulate_step(&mut grid);
        steps += 1;
    }
    
    steps
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
