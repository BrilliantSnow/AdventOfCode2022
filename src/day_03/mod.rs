use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;

#[derive(Clone)]
struct Rucksack {
    all: String,
    left: String,
    right: String,
}

impl Rucksack {
    fn new(all_contents: String) -> Rucksack {
        let mut all_characters = all_contents.chars();
        let count = all_contents.chars().count();
        Rucksack {
            all: all_contents.to_owned(),
            left: all_characters.by_ref().take(count / 2).collect(),
            right: all_characters.take(count / 2).collect(),
        }
    }
}

pub fn main() {
    let filename = "src/day_03/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut storage_list: Vec<Rucksack> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        storage_list.push(Rucksack::new(line));
    }
    let mut count = 0;
    // part 1
    for ruck_sack in &storage_list {
        'outer: for left in ruck_sack.left.chars() {
            for right in ruck_sack.right.chars() {
                if left == right {
                    count += get_priority(left);
                    break 'outer;
                }
            }
        }
    }
    println!("(Part 1) The total priority of misplaced loot: {}", count);
    // part 2
    let mut elf_group_badges: Vec<char> = vec![];
    // split list of ruck sacks chunks of 3
    for elf_group in storage_list.chunks(3) {
        // find common item between each group of 3
        let common_item = find_common(elf_group.to_vec());
        match common_item {
            Some(x) => elf_group_badges.push(x),
            _ => println!("Bug where elf group did not have a common item"),
        }
    }
    // sum the priorities of all the group badges
    let mut badge_total = 0;
    for badge in elf_group_badges {
        badge_total += get_priority(badge);
    }
    println!("(Part 2) The total priority of the badges: {}", badge_total);
    println!("End of day 03");
}

fn find_common(group: Vec<Rucksack>) -> Option<char> {
    let group_chars: Vec<HashSet<char>> = group.iter().map(|x| x.all.chars().collect()).collect();
    let match_chars: HashSet<&char> = group_chars[0].iter().collect();
    // iterates over the first sack's letters. Could be cheaper if you used the shortest of the three
    for character in match_chars.iter() {
        // checks if all the other sacks contain the same character
        if group_chars[1..].iter().all(|contents| contents.contains(character)) {
            return Some(**character);
        }
    }
    return None;
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
