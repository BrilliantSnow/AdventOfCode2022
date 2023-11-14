use std::{
    cell::RefCell,
    error::Error,
    io::{BufRead, BufReader},
    ops::Deref,
    rc::{Rc, Weak},
};

pub enum Type {
    Dir,
    File,
}

pub struct Node {
    pub r#type: Type,
    pub name: String,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>,
    pub size: Option<i128>,
}

pub trait NodeModify: Sized {
    fn add_file(&self, name: &str, size: i128);
    fn add_directory(&self, name: &str);
    fn handle_cd(&self, command: Option<&str>) -> Option<Self>;
    fn calculate_size(&self) -> i128;
    fn collect(&self) -> Vec<Self>;
    fn collect_dir_sizes(&self) -> Vec<i128>;
}

impl NodeModify for Rc<RefCell<Node>> {
    fn add_file(&self, name: &str, size: i128) {
        let file = Node {
            children: vec![],
            name: name.to_string(),
            parent: Some(Rc::downgrade(self)),
            size: Some(size),
            r#type: Type::File,
        };
        self.borrow_mut().children.push(Rc::new(RefCell::new(file)));
    }

    fn add_directory(&self, name: &str) {
        let directory = Node {
            children: vec![],
            name: name.to_string(),
            parent: Some(Rc::downgrade(self)),
            size: None,
            r#type: Type::Dir,
        };
        self.borrow_mut()
            .children
            .push(Rc::new(RefCell::new(directory)));
    }

    fn handle_cd(&self, command: Option<&str>) -> Option<Self> {
        match command {
            Some("..") => match &self.deref().borrow_mut().parent {
                Some(parent) => Some(parent.upgrade().expect("Parent to not be borrowed")),
                None => None,
            },
            Some("/") => None,
            Some(file_name) => self
                .borrow_mut()
                .children
                .iter()
                .find(|child| child.deref().borrow().name == file_name)
                .cloned(),
            None => panic!("Called cd with no arguments"),
        }
    }

    fn calculate_size(&self) -> i128 {
        if let Type::Dir = self.borrow().r#type {
            let size = self
                .borrow()
                .children
                .iter()
                .map(|child| child.calculate_size())
                .sum();
            // println!("Directory size: {}", size);
            return size;
        } else {
            // println!("File size: {}", self.borrow().size.unwrap_or(0));
            return self.borrow().size.expect("Files have sizes");
        }
    }

    fn collect(&self) -> Vec<Self> {
        let mut flat_child_list: Vec<Self> = self
            .borrow()
            .children
            .iter()
            .map(|child| child.collect())
            .flatten()
            .collect();
        flat_child_list.push(self.clone());
        return flat_child_list;
    }

    fn collect_dir_sizes(&self) -> Vec<i128> {
        let mut output: Vec<i128> = self
            .borrow()
            .children
            .iter()
            .filter(|child| matches!(child.borrow().r#type, Type::Dir))
            .map(|child| child.collect_dir_sizes())
            .flatten()
            .collect();
        output.push(self.calculate_size());
        return output;
    }
}

pub fn main() {
    let filename = "src/day_07/input.txt";
    let file = std::fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // root
    let root = Rc::new(RefCell::new(Node {
        r#type: Type::Dir,
        name: "".to_string(),
        parent: None,
        children: vec![],
        size: None,
    }));
    let mut current = root.clone();
    // loop through commands line by line
    for (_, maybe_line) in reader.lines().enumerate() {
        if let Ok(line) = maybe_line {
            // println!("Line number: {}: {}", index, line);
            let tokens = line.split_whitespace();
            let mut tokens = tokens.clone();
            match tokens.next() {
                Some("$") => match tokens.next() {
                    Some("ls") => {
                        // do nothing. prints out files. The files will be read on the following lines
                    }
                    Some("cd") => match current.handle_cd(tokens.next()) {
                        Some(next) => {
                            current = next;
                        }
                        None => current = root.clone(),
                    },
                    Some(invalid_command) => {
                        panic!(
                            "{} is not a valid command. Commands are: [cd, ls]",
                            invalid_command
                        );
                    }
                    None => (),
                },
                Some("dir") => {
                    match tokens.next() {
                        Some(dir_name) => {
                            current.add_directory(dir_name);
                        }
                        None => {
                            // hmmm
                        }
                    }
                }
                Some(data) => {
                    if let Ok(file_size) = data.parse::<i128>() {
                        match tokens.next() {
                            Some(file_name) => {
                                current.add_file(file_name, file_size);
                            }
                            None => {
                                panic!("ls returned a file size without a diretory/file name");
                            }
                        }
                    } else {
                        panic!("{} is not a valid file size", data);
                    }
                }
                None => (),
            }
        } else {
            panic!("Error with reading input file");
        }
    }
    let _ = current;
    let flat_list = root.collect_dir_sizes();
    // part 1
    let large_directories_size: i128 = flat_list.iter().filter(|size| **size <= 100000).sum();
    println!("Part 1 answer: {}", large_directories_size);
    // part 2
    let total_disk_size = 70000000;
    let needed_space = 30000000;
    let used_space: i128 = root.calculate_size();
    let space_needed_to_free = needed_space - (total_disk_size - used_space);
    let smallest_directory_large_enough = flat_list
        .iter()
        .filter(|size| **size >= space_needed_to_free)
        .min()
        .expect("List has at least 1 item");
    println!("Part 2 answer: {}", smallest_directory_large_enough);
    println!("End of day 07");
}
