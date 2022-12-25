use regex::Captures;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct CargoShip {
    cargo_rows: Vec<Vec<String>>,
}

impl CargoShip {
    fn new() -> CargoShip {
        CargoShip {
            cargo_rows: vec![vec![]; 9],
        }
    }
    fn transfer(&mut self, amount: usize, from: usize, to: usize) {
        for _ in 0..amount {
            match self.cargo_rows[from - 1].pop() {
                Some(cargo) => {
                    self.cargo_rows[to - 1].push(cargo);
                }
                None => {}
            }
        }
    }
    fn get_top_crates(&self) -> Vec<String> {
        let mut output: Vec<String> = vec![];
        for column in &self.cargo_rows {
            output.push(match column.last() {
                Some(cargo) => cargo.to_owned(),
                None => " ".to_owned(),
            });
        }
        output
    }
    fn display(&self) {
        let mut tallest = 0;
        for row in &self.cargo_rows {
            if row.len() > tallest {
                tallest = row.len();
            }
        }
        for index in (0..tallest).rev() {
            for cargo_column in &self.cargo_rows {
                match cargo_column.get(index) {
                    Some(cargo) => print!("{} ", cargo),
                    None => print!("    "),
                }
            }
            println!();
        }
    }
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut ship = CargoShip::new();
    let cargo_pattern = Regex::new(r"(?:\[(\w)\]|(\s{3}))").unwrap();
    let move_pattern = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    let mut file_iter = reader.lines();

    for line in file_iter.by_ref().take(10) {
        let line = line.unwrap();
        // build cargoship
        for (index, token) in cargo_pattern.find_iter(&line).enumerate() {
            if !token.as_str().trim().is_empty() {
                ship.cargo_rows[index].push(token.as_str().to_owned());
            }
        }
    }
    for row in &mut ship.cargo_rows {
        row.reverse();
    }

    // commit crane move
    for line in file_iter {
        let line = line.unwrap();
        let specifiers: Vec<Captures> = move_pattern.captures_iter(&line).collect();
        ship.transfer(
            specifiers[0].get(1).unwrap().as_str().parse().unwrap(),
            specifiers[0].get(2).unwrap().as_str().parse().unwrap(),
            specifiers[0].get(3).unwrap().as_str().parse().unwrap(),
        )
    }
    ship.display();
    for mut top_box in ship.get_top_crates() {
        print!("{}", top_box.remove(1));
    }
    println!();
}
