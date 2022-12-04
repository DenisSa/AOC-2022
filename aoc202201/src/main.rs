use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("/home/d/Development/AOC-2022/aoc202201/src/input.txt")?;
    let reader = BufReader::new(file);
    let mut current_calorie_tracker = 0;
    let mut vec = Vec::new();

    for line in reader.lines() {
        let parse_result = line?.trim().parse::<i32>();
        match parse_result{
            Ok(num) => current_calorie_tracker += num,
            Err(_e) => {
                vec.push(current_calorie_tracker);
                current_calorie_tracker = 0;
            },
        } 
    }
    vec.sort();
    let top_three: i32 = (&vec[vec.len()-3..]).iter().sum();
    let top_one: i32 = vec[vec.len()-1];
    println!("Part 1: {top_one}");
    println!("Part 2: {:?}", top_three);
    Ok(())
}