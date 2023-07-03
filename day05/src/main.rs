use std::fs;
use std::num::ParseIntError;

fn main() {

    let input_path =  "input.txt";

    let content:String = fs::read_to_string(input_path)
        .expect("Error loading the file!");

    let lines:Vec<&str> = content.lines().collect();


    // Part 1 & 2
    if let Some(split_idx) = lines.iter().position(|line| {
        line.is_empty()
    }) {
        let tower: Vec<_> = lines[..split_idx].to_vec();
        let n = tower[tower.len() -1].chars().filter(|c| c.is_numeric()).count();

        let mut queues_e1: Vec<Vec<char>> = vec![Vec::new(); n];
        let mut queues_e2: Vec<Vec<char>> = vec![Vec::new(); n];

        tower[..tower.len() - 1]
            .iter()
            .rev()
            .for_each(|line| {
                line.chars()
                    .skip(1)
                    .enumerate()
                    .filter(|(i, _)| { i%4 == 0 })
                    .map(|(_,c)| {c})
                    .enumerate()
                    .for_each(|(i,c)|{
                        if c != ' ' {
                            queues_e1[i].push(c);
                            queues_e2[i].push(c);
                        }
                    });
            });

        lines[split_idx+1..lines.len()]
            .iter()
            .map(|instruction| { parse_instruction(instruction) })
            .for_each(|instruction|{
                match instruction {
                    Ok((ammount, origin, destiny)) => {
                        for _ in 0..ammount {
                            let item = queues_e1[origin-1].pop();
                            queues_e1[destiny-1].push(item.unwrap());
                            }

                        let len = queues_e2[origin-1].len();
                        let end_index = len - 1;
                        let start_index = end_index.saturating_sub(ammount - 1);

                        let removed_slice = queues_e2[origin - 1]
                            .splice(start_index..=end_index, Vec::new())
                            .collect::<Vec<_>>();

                        queues_e2[destiny - 1].extend(removed_slice);
                        }
                    Err(err) => {
                        println!("Error when loading the instructions : {}", err)
                        }
                }
            });
        print!("Part 1 Answer: ");
        queues_e1.iter().for_each(|t| {
            print!("{}", t.last().unwrap());
        });
        println!("");
        print!("Part 2 Answer: ");
        queues_e2.iter().for_each(|t| {
            print!("{}", t.last().unwrap());
        });
        println!("");
    } else {
        println!("Error bro. :(");
    }

}

fn parse_instruction(instruction: &str) -> Result<(usize, usize, usize), ParseIntError> {
    let values: Vec<&str>= instruction.split(" ").collect();

    let ammount:usize = values[1].parse::<usize>()?;
    let origin:usize =  values[3].parse::<usize>()?;
    let destiny:usize = values[5].parse::<usize>()?;

    Ok((ammount, origin, destiny))


}
