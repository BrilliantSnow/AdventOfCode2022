use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_first_instance_of_n_unique(file: File, n: usize) -> usize {
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut index = n;
        if let Ok(input) = line {
            let letters: Vec<char> = input.chars().collect();
            for four_letters in letters.windows(n) {
                let set: HashSet<&char> = HashSet::from_iter(four_letters);
                if set.len() == n {
                    return index;
                } else {
                    index += 1;
                }
            }
        }
    }
    return 0;
}

pub fn main() {
    let filename = "src/day_06/input.txt";
    let file = File::open(filename).unwrap();
    println!("Part 1 solution: {}", find_first_instance_of_n_unique(file, 4));
    let file = File::open(filename).unwrap();
    println!("Part 2 solution: {}", find_first_instance_of_n_unique(file, 14));
}
