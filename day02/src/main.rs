use std::fs;

fn main() {
    // Part 1
    let file_path = "input.txt";

    let content = fs::read_to_string(file_path)
        .expect("Failed to read the file {file_path}");

    let sum_e1: i32 = content
        .lines()
        .map(|line| {
            let players: Vec<&str> = line.split(" ").collect();
            rock_paper_scissors_e1(players[0], players[1]).unwrap()
        }).sum();

    println!("1. What would your total score be if everything goes exactly according to your strategy guide? ");
    println!("\t- The total score would be \x1b[1m{sum_e1}\x1b[0m.");

    // Part 2
    let sum_e2: i32 = content
        .lines()
        .map(|line| {
            let players: Vec<&str> = line.split(" ").collect();
            rock_paper_scissors_e2(players[0], players[1]).unwrap()
        }).sum();

    println!("2. Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide? ");
    println!("\t- The total score would be \x1b[1m{sum_e2}\x1b[0m.");

}

// A <= B <= C
// 1    2    3
// X <= Y <= Z

fn rock_paper_scissors_e2(player1: &str, player2:&str) -> Result<i32, &'static str> {
   match player2 {
       "X" => match player1 { // MUST LOSE
           "A" => Ok(3 + 0), 
           "B" => Ok(1 + 0),
           "C" => Ok(2 + 0),
           _ => Err("Invalid input for player1")
       },
       "Y" => match player1 { // MUST DRAW
           "A" => Ok(1 + 3),
           "B" => Ok(2 + 3),
           "C" => Ok(3 + 3),
           _ => Err("Invalid input for player1")
       },
       "Z" => match player1 { // MUST WIN
           "A" => Ok(2 + 6),
           "B" => Ok(3 + 6),
           "C" => Ok(1 + 6),
           _ => Err("Invalid input for player1")
       },
        _ => Err("Invalid input for player2")
   } 
}

fn rock_paper_scissors_e1(player1: &str, player2: &str) -> Result<i32, &'static str> {
    match (player1, player2) {
        ("A", "X") => Ok(1 + 3),           
        ("A", "Y") => Ok(2 + 6),           
        ("A", "Z") => Ok(3 + 0),           
        ("B", "X") => Ok(1 + 0),           
        ("B", "Y") => Ok(2 + 3),           
        ("B", "Z") => Ok(3 + 6),           
        ("C", "X") => Ok(1 + 6),           
        ("C", "Y") => Ok(2 + 0),           
        ("C", "Z") => Ok(3 + 3),           
        _ => Err("Invalid input for players"),
    }
}
