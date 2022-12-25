use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut found_four = 0;
    let mut found_fourteen = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut last_four: [char;4] = ['R', 'u', 's', 't'];
        let mut last_fourteen: [char;14] = ['F';14];
        for (index, character) in line.chars().enumerate() {

            last_four[index % 4] = character;
            if index > 3 {
                if HashSet::from(last_four).len() == 4 && found_four == 0 {
                    found_four = index + 1;
                }
            }

            last_fourteen[index % 14] = character;
            if index > 13 {
                if HashSet::from(last_fourteen).len() == 14 && found_fourteen == 0 {
                    found_fourteen = index + 1;
                }
            }

            if found_fourteen != 0 && found_four != 0 {
                break;
            }
        }
    }

    println!("Part 1: {}", found_four);
    println!("Part 2: {}", found_fourteen);
}
