use std::fs;
use std::collections::HashSet;

fn main() {

    let file_path = "input.txt";

    let content = fs::read_to_string(file_path).expect("Error loading the file :(").replace("\n","");

    // let chunk_size = 4;  let part = 1;    // Part 1
    let chunk_size = 14;  let part = 2;   // Part 2
    let mut first_stop = false;

    for i in 0..=content.len() - chunk_size {
        let chunk = &content[i..i+chunk_size];
        if check_repeat(chunk) && !first_stop  {
            first_stop = true
        } else if !check_repeat(chunk) {
            println!("Part {} Answer: {}", part,  i+chunk_size);
            break;
        }
    }
}


fn check_repeat(chunk: &str) -> bool {

    let mut chars_seen = HashSet::new();

    for c in chunk.chars() {
        if !chars_seen.insert(c) {
            return true;
        }
    }
    return false;
}
