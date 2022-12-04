use std::{fs::File, io::{BufReader, self, BufRead, Read}};

fn main_pt1() -> io::Result<()> {
    let file = File::open("/home/d/Development/AOC-2022/aoc202203/src/input.txt")?;
    let reader = BufReader::new(file);
    let mut letter_collection = Vec::new();
    let mut total_sum:u32 = 0;

    for line in reader.lines(){
        match line {
            Ok(s) => {
                let s_half_len = s.len() / 2;
                let item1 = &s[..s_half_len];
                let item2 = &s[s_half_len..];
                
                for c in item1.chars(){
                    if item2.contains(c){
                        letter_collection.push(c);
                        break;
                    }
                }
            },
            Err(_e) => {}
        }
    }

    for letter in letter_collection {
        let converted_letter = letter as u32;
        if converted_letter >= 97 { //lower case
            total_sum += converted_letter - 96;
        }
        else { //upper case
            total_sum += converted_letter - 38;
        }
    }
    println!("Total: {}", total_sum);
    Ok(())
}

fn main() -> io::Result<()> {
    let file = File::open("/home/d/Development/AOC-2022/aoc202203/src/input.txt")?;
    let reader = BufReader::new(file);
    let mut letter_collection = Vec::new();
    let mut total_sum:u32 = 0;

    let lines: Vec<_> = reader.lines().collect();

    for chunk in lines.chunks(3){
        for c in chunk[0].as_ref().unwrap().chars(){
            if chunk[1].as_ref().unwrap().contains(c) && chunk[2].as_ref().unwrap().contains(c){
                letter_collection.push(c);
                break;
            }
        }
    }

    println!("{:?}", letter_collection);

    for letter in letter_collection {
        let converted_letter = letter as u32;
        if converted_letter >= 97 { //lower case
            total_sum += converted_letter - 96;
        }
        else { //upper case
            total_sum += converted_letter - 38;
        }
    }

    println!("Total: {}", total_sum);
    Ok(())
}