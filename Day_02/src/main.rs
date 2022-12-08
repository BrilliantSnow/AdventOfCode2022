use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_points = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        total_points += points(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap())
    }
    println!("Total points calculated: {}", total_points);
}

fn points(their_move: char, your_move: char) -> i32 {
    let outcome = match (their_move, your_move) {
        // Ties
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,
        // Losses
        ('A', 'Z') => 0,
        ('B', 'X') => 0,
        ('C', 'Y') => 0,
        // Wins
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        _ => panic!("Invalid input {} {}", their_move, your_move),
    };
    let play = match your_move {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => panic!("Invalid move {}", your_move),
    };
    outcome + play
}