use std::env;
use std::fs;

// ANSI escape code for resetting the text color
const RESET: &str = "\x1B[0m";

// ANSI escape codes for different text colors
const RED: &str = "\x1B[31m";
const GREEN: &str = "\x1B[32m";
const YELLOW: &str = "\x1B[33m";
const BLUE: &str = "\x1B[34m";

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Error loading the file");

    let lines: Vec<String> = contents.lines().map(|line| line.trim().to_string()).collect();

    let num_rows = lines.len();
    let num_cols = lines[0].len();

    let mut trees = vec![vec![0; num_cols]; num_rows];
    let mut scenic_score = vec![vec![0; num_cols]; num_rows];

    for (i, line) in lines.iter().enumerate() {
        trees[i] = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    }

    // The edges are always visible
    let mut visible_trees = 2 * num_cols + 2 * (num_rows - 2);

    // Iterate over trees
    for i in 1..(num_rows - 1) {
        for j in 1..(num_cols - 1) {
            let tree_column: Vec<i32> = trees.iter().map(|row| row[j] - trees[i][j]).collect();
            let tree_row: Vec<i32> = trees[i].iter().map(|&val| val - trees[i][j]).collect();

            let routes = vec![
                &tree_column[..i],          // Up
                &tree_column[(i + 1)..],    // Down
                &tree_row[..j],             // Left
                &tree_row[(j + 1)..],       // Right
            ];

            if routes.iter().any(|route| route.iter().all(|&val| val < 0)) {
                visible_trees += 1;
            }

            scenic_score[i][j] = routes
                .iter()
                .enumerate()
                .map(|(i, route)| {
                    viewing_distance(route, i%2 == 0)
                })
                .fold(1, |acc, x| acc * x);
        }
    }

    println!("{YELLOW}P4RT 1{RESET}");
    println!("The Number of {BLUE}visible trees{RESET} is {GREEN}{visible_trees}{RESET}");

    println!("{YELLOW}P4RT 2{RESET}");

    let max_value = scenic_score.iter().max_by_key(|&slice| slice.iter().max());
    match max_value {
        Some(slice) => {
            if let Some(mv) = slice.iter().max() {
                println!("The  highest {BLUE}scenic score posible{RESET} is {GREEN}{mv}{RESET}!");
            }
        }
        None => println!("{RED}The vector is empty!{RESET}"),
    }
}

fn viewing_distance(route: &[i32], rev: bool) -> u32 {
    let mut counter = 0;
    if rev {
        for tree in route.iter().rev() {
            counter += 1;
            if *tree >= 0 { break; }
        }
    } else {
        for tree in route {
            counter += 1;
            if *tree >= 0 { break; }
        }
    }
    counter
}




