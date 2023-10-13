use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn main() {
    let filename = "src/day_02/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut total_points_one = 0;
    let mut total_points_two = 0;

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let their_move = line.chars().nth(0).unwrap();
        let your_move = line.chars().nth(2).unwrap();
        total_points_one += points(their_move, your_move);
        total_points_two += points_two(their_move, your_move);
    }
    println!("(Part 1) Total points calculated: {}", total_points_one);
    println!("(Part 2) Total points calculated: {}", total_points_two);
}

fn points_two(their_move: char, your_move: char) -> i32 {
    fn tie(their_move: char) -> i32 {
        match their_move {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => panic!("Invalid move {}", their_move),
        }
    }
    fn win(their_move: char) -> i32 {
        match their_move {
            'A' => 2,
            'B' => 3,
            'C' => 1,
            _ => panic!("Invalid move {}", their_move),
        }
    }
    fn loss(their_move: char) -> i32 {
        match their_move {
            'A' => 3,
            'B' => 1,
            'C' => 2,
            _ => panic!("Invalid move {}", their_move),
        }
    }
    let outcome = match (their_move, your_move) {
        // Ties
        ('A', 'Y') => 3 + tie('A'),
        ('B', 'Y') => 3 + tie('B'),
        ('C', 'Y') => 3 + tie('C'),
        // Losses
        ('A', 'X') => 0 + loss('A'),
        ('B', 'X') => 0 + loss('B'),
        ('C', 'X') => 0 + loss('C'),
        // Wins
        ('A', 'Z') => 6 + win('A'),
        ('B', 'Z') => 6 + win('B'),
        ('C', 'Z') => 6 + win('C'),

        _ => panic!("Invalid input {} {}", their_move, your_move),
    };
    outcome
}

fn points(their_move: char, your_move: char) -> i32 {
    let outcome = match (their_move, your_move) {
        // Ties
        ('A', 'X') |
        ('B', 'Y') |
        ('C', 'Z') => 3,
        // Losses
        ('A', 'Z') |
        ('B', 'X') |
        ('C', 'Y') => 0,
        // Wins
        ('A', 'Y') |
        ('B', 'Z') |
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