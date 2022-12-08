use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Rucksack {
    left: String,
    right: String,
}

impl Rucksack {
    fn new(all_contents: String) -> Rucksack {
        let mut all_characters = all_contents.chars();
        let count = all_contents.chars().count();
        Rucksack {
            left: all_characters.by_ref().take(count / 2).collect(),
            right: all_characters.take(count / 2).collect(),
        }
    }
}

fn main() {
    let filename = "src/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut storage_list: Vec<Rucksack> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        storage_list.push(Rucksack::new(line));
    }
    let mut count = 0;
    for ruck_sack in storage_list {
        'outer: for left in ruck_sack.left.chars() {
            for right in ruck_sack.right.chars() {
                if left == right {
                    count += get_priority(left);
                    break 'outer;
                }
            }
        }
    }
    println!("The total priority of misplaced loot: {}", count);
}

fn get_priority(letter: char) -> u32 {
    match letter as u32 {
        // A-Z, 65-90 mapped to 27-52
        65..=90 => letter as u32 - 38,
        // a-z, 97-122 mapped to 1-26
        97..=122 => letter as u32 - 96,
        // not a-z or A-Z
        _ => panic!("Letter is not [a-zA-Z]"),
    }
}
