use std::{
    collections::HashMap,
    error::Error,
    io::{BufRead, BufReader},
};


pub struct Folder {
    pub name: String,
    pub size: usize,
    pub directories: HashMap<String, Vec<Folder>>,
}

impl Folder {
    pub fn new(name: String) -> Self {
        Folder {
            name,
            size: 0,
            directories: HashMap::new(),
        }
    }
    pub fn new_file(name: String, size: usize) -> Self {
        Folder {
            name,
            size,
            directories: HashMap::new(),
        }
    }
    pub fn child_size(&self) -> usize {
        let mut size = self.size;
        size += self
            .directories
            .iter()
            .map(|(_, sub_directory)| {
                sub_directory
                    .iter()
                    .map(|sub| sub.child_size())
                    .sum::<usize>()
            })
            .sum::<usize>();
        return size;
    }
}

pub struct FileSystem {
    pub file_path: Vec<String>,
    pub files: HashMap<String, Folder>,
}

impl FileSystem {
    pub fn handle_cd(&mut self, command: Option<&str>) {
        if let Some(command) = command {
            println!("cd {}", command);
        }
        match command {
            Some("..") => self.parent_directory(),
            Some("/") => self.root(),
            Some(file) => self.sub_directory(file),
            None => println!("Called cd with no arguments"),
        }
    }

    pub fn add_dir(&mut self, new_directory: Folder) {
        let current_dir = self.file_path_string();
        match self.files.get(&current_dir) {
            Some(existing_dir) => {
                // dont add it
            }
            None => {
                self.files.insert(
                    format!("{}/{}", current_dir, new_directory.name),
                    new_directory,
                );
            }
        }
    }

    pub fn register_ls_outputs(&mut self, size: usize, name: &str) {}

    pub fn root(&mut self) {
        self.file_path = vec![];
    }

    pub fn sub_directory(&mut self, file: &str) {
        self.file_path.push(file.into())
    }

    pub fn parent_directory(&mut self) {
        self.file_path.pop();
    }

    pub fn file_path_string(&self) -> String {
        if self.file_path.len() == 0 {
            "/".into()
        } else {
            self.file_path.iter().fold("".into(), |path, word| {
                (&*format!("{}/{}", path, word)).into()
            })
        }
    }
}

struct Directory {
    name: String,
    size: usize,
    folders: HashMap<String, Directory>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let filename = "src/day_07/input.txt";
    // let filename = "testsrc/day_0#/input.txt";
    let file = std::fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // root
    let root_directory = Folder::new("/".into());
    let mut file_system = FileSystem {
        file_path: vec![],
        files: HashMap::new(),
    };
    file_system.add_dir(root_directory);
    // loop through commands line by line
    for (_, maybe_line) in reader.lines().enumerate() {
        if let Ok(line) = maybe_line {
            // println!("Line number: {}: {}", index, line);
            let mut tokens = line.split_whitespace();
            match tokens.next() {
                Some("$") => match tokens.next() {
                    Some("ls") => {
                        // do nothing. prints out files. The files will be read on the following lines
                    }
                    Some("cd") => file_system.handle_cd(tokens.next()),
                    Some(invalid_command) => {
                        return Err(format!(
                            "{} is not a valid command. Commands are: [cd, ls]",
                            invalid_command
                        )
                        .into());
                    }
                    None => (),
                },
                Some("dir") => {}
                Some(data) => {
                    if let Ok(file_size) = data.parse::<usize>() {
                        match tokens.next() {
                            Some(file_name) => {
                                file_system.add_dir(Folder::new_file(file_name.into(), file_size))
                            }
                            None => {
                                return Err(format!(
                                    "ls returned a file size without a diretory/file name"
                                )
                                .into());
                            }
                        }
                    } else {
                        return Err(format!("{} is not a valid file size", data).into());
                    }
                }
                None => (),
            }
        } else {
            return Err(format!("Error with reading input file").into());
        }
    }
    let mut flat_list: Vec<(String, Folder)> = file_system
        .files
        .into_iter()
        .map(|(path, data)| {
            (
                path.get(0..data.name.len())
                    .expect("file path contains file name")
                    .to_string(),
                data,
            )
        })
        .collect();
    flat_list.sort_by(|(left, _), (right, _)| left.cmp(right));
    let mut directories: HashMap<&str, Vec<&Folder>> = HashMap::new();
    // flat list to map of folders. Paths are flat
    for (path, data) in flat_list.iter() {
        directories
            .entry(path)
            .or_insert(Vec::new())
            .push(data);
    }
    for (file, contents) in directories.iter() {
        println!("Directory: {}", file);
        for folder in contents {
            println!("\t{}: {}", folder.name, folder.size);
        }
    }
    println!("End of day 07");
    Ok(())
}
