use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = "src/input.txt";
    // let filename = "src/testinput.txt";
    let file = std::fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    day_07(reader);
}

struct File {
    size: i32,
    name: String,
}

struct Directory {
    files: Vec<File>,
    subdirectories: Vec<Directory>,
    name: String,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            name: "root".to_string(),
            files: vec![],
            subdirectories: vec![],
        }
    }

    fn get_subdir(&mut self, sub_dir: String) -> Option<&mut Directory> {
        self.subdirectories.iter_mut().find(|x| x.name == sub_dir)
    }

    fn add_subdir(&mut self, sub_dir: Directory) {
        self.subdirectories.push(sub_dir);
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn size(&self) -> i32 {
        let this_size: i32 = self.files.iter().map(|x| x.size).sum();
        let children_size: i32 = self.subdirectories.iter().map(|x| x.size()).sum();
        let size = this_size + children_size;
        // if size <= 100000 {
        //     println!("{}", size);
        // }
        return size;
    }

    fn print(&self, indent: &str) {
        println!("{}Dir: {} {}", indent, self.name, self.size());
        for file in &self.files {
            println!("{}{} {}", indent.to_owned() + "  ", file.name, file.size);
        }
        for dir in &self.subdirectories {
            dir.print((indent.to_owned() + "  ").as_str());
        }
    }
}

fn day_07(reader: BufReader<std::fs::File>) {
    let mut root_directory = Directory::new();

    let mut current_directory = &mut root_directory;
    let mut directory_stack: VecDeque<*mut Directory> = VecDeque::new();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                let mut token = input.split(' ').fuse();
                match token.next() {
                    Some("dir") => {
                        // todo
                    }
                    Some("$") => {
                        match token.next() {
                            Some("cd") => match token.next() {
                                Some("..") => unsafe {
                                    current_directory = &mut *directory_stack.pop_back().unwrap();
                                },
                                Some("/") => {
                                    current_directory = &mut root_directory;
                                    directory_stack.clear();
                                }
                                Some(dir) => {
                                    match current_directory.get_subdir(dir.to_string()) {
                                        Some(existing_directory) => {
                                            directory_stack.push_back(existing_directory);
                                        }
                                        None => {
                                            current_directory.add_subdir(Directory {
                                                files: vec![],
                                                subdirectories: vec![],
                                                name: dir.to_string(),
                                            });
                                            directory_stack.push_back(
                                                current_directory
                                                    .get_subdir(dir.to_string())
                                                    .unwrap(),
                                            );
                                        }
                                    }
                                    current_directory =
                                        current_directory.get_subdir(dir.to_string()).unwrap();
                                }
                                None => todo!(),
                            },
                            Some("ls") => {
                                // not sure this needs handled
                            }
                            _ => todo!("Handle additional conditions"),
                        }
                    }
                    Some(size) => {
                        let file_size: i32 = size.parse().unwrap();
                        current_directory.add_file(File {
                            size: file_size,
                            name: token.next().unwrap().to_string(),
                        });
                    }
                    None => todo!("Handle additional conditions"),
                }
            }
            Err(error) => panic!("Reading line from file failed: {}", error),
        }
    }
    root_directory.print("");
    println!("{}", root_directory.size());
}
