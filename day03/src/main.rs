use std::fs;

fn main() {

    // Part 1
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Error loading the file!");

    let sum_e1:u32 = contents
        .lines()
        .map(|line| {
            let indx = line.len()/2;

            let first_halve = &line[..indx];
            let second_halve = &line[indx..];

            let mut found_match: bool = false;

            let val:u32 = first_halve
                .chars()
                .filter_map(|ch| {
                    if !found_match && second_halve.contains(ch) {
                        found_match = true;
                        let priority = to_number(ch);
                        Some(priority)
                    } else {
                        None
                    }
            }).sum();

            val
        }).sum();

    println!("1. Find the item type that appears in both compartments of each rucksack. What is the sum of the priorities of those item types?");
    println!("\t- the sum of the priorities of those item types would be \x1b[1m{sum_e1}\x1b[0m.");

    // Part 2
    let lines: Vec<&str> = contents.lines().collect();

    let sum_e2:u32 = lines
        .chunks(3)
        .map( |group| {
            let e1 = group.iter().min_by_key(|&line| line.len()).unwrap();
            let mut rem = group.iter().filter(|&line| line != e1);

            let e2 = rem.next().unwrap();
            let e3 = rem.next().unwrap();

            let mut found_match: bool = false;

            let val:u32 = e1
                .chars()
                .filter_map(|ch| {
                    if !found_match && e2.contains(ch) && e3.contains(ch) {
                        found_match = true;
                        Some(to_number(ch))
                    } else {
                        None
                    }
                }).sum();
            val
        }).sum();

    println!("2. Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types? ");
    println!("\t- the sum of the priorities of those item types would be \x1b[1m{sum_e2}\x1b[0m.");

}

fn to_number(ch: char) -> u32 {
    match ch {
        'a'..='z' => (ch as u8 - b'a' + 1)  as u32,
        'A'..='Z' => (ch as u8 - b'A' + 27) as u32,
        _ => 0,
    }
}
