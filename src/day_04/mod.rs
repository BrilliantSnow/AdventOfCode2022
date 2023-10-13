use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Pair {
    first: [i32; 2],
    second: [i32; 2],
}

impl Pair {
    fn new(input: String) -> Pair {
        let pairs: Vec<&str> = input.split(",").collect();
        Pair {
            first: pairs[0]
                .split("-")
                .map(|x| x.to_string().parse().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap(),
            second: pairs[1]
                .split("-")
                .map(|x| x.to_string().parse().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap(),
        }
    }
}

pub fn main() {
    let filename = "src/day_04/input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut assignment_pairs: Vec<Pair> = vec![];

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        assignment_pairs.push(Pair::new(line));
    }

    let mut overlap_count = 0;

    for pair in &assignment_pairs {
        let first_length = pair.first[1] - pair.first[0];
        let second_length = pair.second[1] - pair.second[0];

        if first_length > second_length {
            // first is a superset of second
            if pair.first[0] <= pair.second[0] && pair.first[1] >= pair.second[1] {
                overlap_count += 1;
            }
        } else if second_length > first_length {
            // second is a superset of first
            if pair.first[0] >= pair.second[0] && pair.first[1] <= pair.second[1] {
                overlap_count += 1;
            }
        } else {
            // they are the same set
            if pair.first == pair.second {
                overlap_count += 1;
            }
        }
    }
    println!("Part 1: there are {} pairs that completely overlap!", overlap_count);

    let mut overlaps_at_all = 0;
    // Part 2
    for pair in &assignment_pairs {
        fn is_between(lower: [i32;2], upper: [i32;2]) -> bool {
            let contains_start = lower[0] >= upper[0] && lower[0] <= upper[1];
            let contains_end = lower[1] >= upper[0] && lower[1] <= upper[1];
            return contains_start || contains_end;
        }
        if is_between(pair.first, pair.second) || is_between(pair.second, pair.first) {
            overlaps_at_all += 1;
        }
    }

    println!("Part 2: there are {} pairs that overlap at all!", overlaps_at_all);
}
