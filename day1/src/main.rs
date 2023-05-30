use std::fs;
use std::string::String;


fn main() {
    // Part 1
    let file_path = String::from("input.txt");
    let mut sorted_array: Vec<i32> = Vec::new();

    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read file!");

    let chunks: Vec<&str> = content.split("\n\n").collect();

    for chunk in chunks {
        let sum: i32 = chunk
            .split("\n")
            .filter(|&line| !line.trim().is_empty())
            .map(|num| num.parse::<i32>().unwrap())
            .sum();
        append_sorted(&mut sorted_array, sum);
    }

    let calories = *sorted_array.last().unwrap();

    println!("1. Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?");
    println!("\t- The elf is carrying \x1b[1m{calories}\x1b[0m calories.");

    // Part 2
    let top_three = &sorted_array[sorted_array.len() - 3..sorted_array.len()];
    let top_three_sum:i32 = top_three.iter().sum();

    println!("2. Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total? ");
    println!("\t- The top three are carrying \x1b[1m{top_three_sum}\x1b[0m calories in total!");
}

fn append_sorted(arr: &mut Vec<i32>, element: i32) {
    
    let index = match arr.binary_search(&element) {
        Ok(index) => index,
        Err(index) => index,
    };

    arr.insert(index, element);
}
