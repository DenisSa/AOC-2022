use std::{fs::File, io::{BufReader, self, BufRead}, cmp};

fn main_p1() -> io::Result<()> {
    let file = File::open("/home/d/Development/AOC-2022/aoc-202204/src/input.txt")?;
    let reader = BufReader::new(file);
    let mut counter = 0;

    for line in reader.lines(){
        let line_s = line?;
        let groups: Vec<&str> = line_s.split(",").collect();
        let group1: Vec<&str> = groups[0].split("-").collect();
        let group2: Vec<&str> = groups[1].split("-").collect();

        let digit1_g1 = group1[0].parse::<i32>().unwrap();
        let digit2_g1 = group1[1].parse::<i32>().unwrap();
        let digit1_g2 = group2[0].parse::<i32>().unwrap();
        let digit2_g2 = group2[1].parse::<i32>().unwrap();

        if digit1_g1 <= digit1_g2 && digit2_g1 >= digit2_g2 ||
        digit1_g1 >= digit1_g2 && digit2_g1 <= digit2_g2
        {
            counter+=1;
        }        
    }

    println!("{}", counter);

    Ok(())
}

fn main() -> io::Result<()> {
    let file = File::open("/home/d/Development/AOC-2022/aoc-202204/src/input.txt")?;
    let reader = BufReader::new(file);
    let mut counter = 0;

    for line in reader.lines(){
        let line_s = line?;
        let groups: Vec<&str> = line_s.split(",").collect();
        let group1: Vec<&str> = groups[0].split("-").collect();
        let group2: Vec<&str> = groups[1].split("-").collect();

        let digit1_g1 = group1[0].parse::<i32>().unwrap();
        let digit2_g1 = group1[1].parse::<i32>().unwrap();
        let digit1_g2 = group2[0].parse::<i32>().unwrap();
        let digit2_g2 = group2[1].parse::<i32>().unwrap();

        if cmp::max(digit1_g1, digit1_g2) <= cmp::min(digit2_g1, digit2_g2){
            counter+=1;
        }    
    }

    println!("{}", counter);

    Ok(())
}
