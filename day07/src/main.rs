use std::env;
use std::fs;
use std::collections::HashMap;

// ANSI escape code for resetting the text color
const RESET: &str = "\x1B[0m";

// ANSI escape codes for different text colors
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
const BLUE: &str = "\x1B[34m";


#[derive(Debug, Clone)]
struct File {
    name: String,
    size: u64,
}

impl File {
    fn new(name: String, size: u64) -> Self {
        Self {
            name,
            size
        }
    }
}

#[derive(Debug, Clone)]
struct Folder {
    name: String,
    folders: HashMap<String, Folder>,
    files: HashMap<String, File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name, 
            folders: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_folder(&mut self, name: String, folder: Folder) {
        self.folders.insert(name, folder);
    }

    fn add_file(&mut self, name: String, file: File) {
        self.files.insert(name, file);
    }

    fn tree(&self, depth: usize) {
        println!("{}|─> {}{} (dir){}", "|   ".repeat(depth), BLUE, self.name, RESET);

        for file in self.files.values() {
            println!("{}|── {}{} (size={}){}", "|   ".repeat(depth+1), YELLOW, file.name, file.size, RESET);
        }

        for folder in self.folders.values() {
            folder.tree(depth+1);
        }

    }

    fn get_tot_size(&self) -> u64 {
        let folder_size: u64 = self.folders.values().map(|folder| folder.get_tot_size()).sum();
        let files_size: u64 = self.files.values().map(|file| file.size).sum();
        folder_size + files_size

    }
    
    fn size_large_folders(&self, max_size: u64) -> u64 {
        let mut size = 0;

        if self.get_tot_size() <= max_size {
            size += self.get_tot_size();
        } 

        for folder in self.folders.values() {
            size += folder.size_large_folders(max_size);
        } 
        size
    }

    fn min_to_fit(&self, min_needed: u64) -> u64 {
        let mut folder_size = self.get_tot_size();

        for folder in self.folders.values() {
            let fz = folder.get_tot_size();
            if fz >= min_needed && fz < folder_size {
                folder_size = folder.min_to_fit(min_needed)
            }
        }
        folder_size
    }

}


#[derive(Debug)]
struct FileSystem {
    root: Folder,
    current_path: Vec<String>,
}

impl FileSystem {
    fn new() -> Self {
        Self {
            root: Folder::new("/".to_string()),
            current_path: vec!["/".to_string()],
        }
    }

    fn cd(&mut self, val: &str) {
        match val {
            ".." => {
                if self.current_path.len() > 1 {
                    self.current_path.pop();
                }
            },
            _ => {
                self.current_path.push(val.to_string());
            },
        }
    }

    fn mkdir(&mut self, name: String) {
        let mut current_folder = &mut self.root;

        for folder_name in &self.current_path[1..] {
            match current_folder.folders.get_mut(folder_name) {
                Some(folder) => {
                    current_folder = folder;
                }
                None => {
                    println!("Path not found: {}", folder_name);
                    return;
                }
            }
        }

        let new_folder = Folder::new(name.clone());
        current_folder.add_folder(name.clone(), new_folder);
    }

    fn mkfile(&mut self, name: String, size: u64) {
        let mut current_folder = &mut self.root;

        for folder_name in &self.current_path[1..] {
            match current_folder.folders.get_mut(folder_name) {
                Some(folder) => {
                    current_folder = folder;
                }
                None => {
                    println!("Path not found: {}", folder_name);
                    return;
                }
            }
        }

        let new_file = File::new(name.clone(), size);
        current_folder.add_file(name.clone(), new_file);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Error loading file!");

    let mut sys = FileSystem::new();

    contents
        .split("$ ")
        .skip(2)
        .for_each(|instruction| {
            if instruction.len() >= 2 {

                match &instruction[..2] {
                    "ls" => {
                        let output = &instruction[3..instruction.len()];
                        output
                            .lines()
                            .for_each(|line| {
                                let (is_file, name, size) = pars3_file(line)
                                    .unwrap_or((false, "ERROR", 0));
                                if is_file {
                                    sys.mkfile(name.to_string(), size);
                                } else {
                                    sys.mkdir(name.to_string());
                                }
                            })
                    },
                    "cd" => {
                        let output = &instruction[3..instruction.len()];
                        sys.cd(output.trim_end_matches('\n'));
                    },
                    _ => {
                        println!("No matching instruction");
                    }
                }

            }
        });

    sys.root.tree(0);

    // PART 1
    // let lim = 100000;
    // println!("The combined size of all the folders with size higher than {}{}{} is {}{}", RED, lim, RESET, GREEN, sys.root.size_large_folders(lim));
    
    // PART 2
    let availability = 70000000;
    let needs = 30000000;
    let root_size = sys.root.get_tot_size();

    let unused_space = availability-root_size;
    let min_needed = needs - unused_space;


    println!("{}", min_needed);
    println!("The min needed to free at least {}{}{} is {}{}{}",RED, min_needed, RESET, GREEN ,sys.root.min_to_fit(min_needed), RESET);
}

fn file_or_dir(value: &str) -> Option<(bool,u64)> {
    if value == "dir" {
        Some((false, 0))
    } else {
        Some((true, value.parse::<u64>().unwrap_or(0)))
    }
}

fn pars3_file(line: &str) -> Option<(bool, &str, u64)> {
    let mut vals = line.split(" ");

    let first = vals.next().expect("Invalid first value!");
    let name = vals.next().expect("Invalid second value!");

    let (is_file, size) = file_or_dir(first).unwrap_or((true, 0));

    Some((is_file, name, size))
}

