use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Elf {
    total_calories: i32,
}

pub fn main() {
    let filename = "src/day_01/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut elf_list: Vec<Elf> = vec![];
    let mut current_elf = Elf { total_calories: 0 };

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            // add current elf to elf list
            elf_list.push(current_elf);
            // next elf
            current_elf = Elf { total_calories: 0 };
        } else {
            // add calories to current elf
            current_elf.total_calories += line.parse::<i32>().unwrap();
        }
    }
    // sort elf list by total calories
    elf_list.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    println!("(Part 1) The most calories carried by a single elf: {}", elf_list.get(0).unwrap().total_calories);

    let mut top_three_count = 0;
    for index in 0..3 {
        top_three_count += elf_list.get(index).unwrap().total_calories;
    }
    println!("(Part 2) The most calories carried between 3 elves: {}", top_three_count);
    println!("End of day 01");
}
