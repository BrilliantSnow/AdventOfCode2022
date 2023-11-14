use std::io::{BufRead, BufReader};

fn all_smaller(trees: Vec<u32>, height: u32) -> bool {
    trees.into_iter().all(|x| x < height)
}

pub fn visible(x: usize, y: usize, list: &Vec<Vec<u32>>) -> bool {
    let tree_height = list[x][y];
    let left: Vec<u32> = (0..x).map(|x| list[x][y]).collect();
    let right: Vec<u32> = (x + 1..99).map(|x| list[x][y]).collect();
    let top: Vec<u32> = (0..y).map(|y| list[x][y]).collect();
    let bottom: Vec<u32> = (y + 1..99).map(|y| list[x][y]).collect();
    let f = |x| all_smaller(x, tree_height);
    return f(left) || f(right) || f(top) || f(bottom);
}

fn view_distance(trees: Vec<u32>, tree_height: u32) -> usize {
    for (index, height) in trees.iter().enumerate() {
        if *height >= tree_height {
            return index + 1;
        }
    }
    return trees.len();
}

pub fn visible_neighbors(x: usize, y: usize, list: &Vec<Vec<u32>>) -> u128 {
    if x == 0 || x == 99 || y == 0 || y == 99 {
        return 0;
    }
    let tree_height = list[x][y];
    let left: Vec<u32> = (0..x).map(|x| list[x][y]).rev().collect();
    let right: Vec<u32> = (x + 1..99).map(|x| list[x][y]).collect();
    let top: Vec<u32> = (0..y).map(|y| list[x][y]).rev().collect();
    let bottom: Vec<u32> = (y + 1..99).map(|y| list[x][y]).collect();
    let f = |list| view_distance(list, tree_height) as u128;
    return f(left) * f(right) * f(top) * f(bottom);
}

pub fn main() {
    let filename = "src/day_08/input.txt";
    let file = std::fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rows: Vec<Vec<u32>> = vec![];
    for (_, maybe_line) in reader.lines().enumerate() {
        if let Ok(line) = maybe_line {
            let letters: Vec<char> = line.chars().collect();
            let number_list = letters
                .iter()
                .map(|letter| letter.to_digit(10).expect("Everything do be a digit"))
                .collect::<Vec<u32>>();
            rows.push(number_list);
        }
    }
    let mut count = 0;
    // not efficient but hey it works
    for x in 0..99 {
        for y in 0..99 {
            if visible(x, y, &rows) {
                count += 1;
            }
        }
    }
    println!("Part 1: {}", count);
    // part 2
    let mut score = vec![];
    for x in 0..99 {
        for y in 0..99 {
            score.push(visible_neighbors(x, y, &rows));
        }
    }
    println!("Part 2: {}", score.iter().max().expect("There to be at least 1"));
}
