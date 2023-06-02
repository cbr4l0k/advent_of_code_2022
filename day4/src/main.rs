use std::fs;


fn main() {

    let file_path = "input.txt";

    let content = fs::read_to_string(file_path)
        .expect("Error loading the file!").replace("\n",",");

    let ranges: Vec<_> =  content[..content.len()-1].split(",").collect();

    // Part 1
    let val_e1: i32 = ranges
        .chunks(2)
        .filter_map(|chunk| {
            let a:Vec<i32> = pars3(chunk[0]);
            let b:Vec<i32> = pars3(chunk[1]);

            if (a[0] >= b[0] && a[1] <= b[1]) || (b[0] >= a[0] && b[1] <= a[1]) {
                Some(1)
            } else {
                None
            }
        }).sum();
    println!("Part 1 Answer: {val_e1}");

    // Part 2
    let val_e2: i32 = ranges
        .chunks(2)
        .filter_map(|chunk| {
            let a:Vec<i32> = pars3(chunk[0]);
            let b:Vec<i32> = pars3(chunk[1]);


            if (a[0] <= b[1]) && (a[1] >= b[0]){
                Some(1)
            } else {
                None
            }
        }).sum();
    println!("Part 2 Answer: {val_e2}");

}

fn pars3(input: &str) -> Vec<i32> {
    let nums: Vec<i32> = input
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    nums
}


