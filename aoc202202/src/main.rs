use std::{fs::File, io::{BufReader, self, BufRead}, collections::HashMap};

fn main_p1() -> io::Result<()> {
    let file = File::open("/home/d/Development/AOC-2022/aoc202202/src/input.txt")?;
    let reader = BufReader::new(file);
    let mut total_score: i32 = 0;

    for line in reader.lines() {
        match line {
            Ok(s) => {
                let move_p1 = i32::from(s.as_bytes()[0] - 64);
                let move_p2 = i32::from(s.as_bytes()[2] - 87);

                let move_sub = move_p1 - move_p2;

                let match_score = match move_sub {
                    -2 => move_p2,
                    -1 => 6 + move_p2,
                    0 =>  3 + move_p2,
                    1 => move_p2,
                    2 => 6 + move_p2,
                    _ => 0,
                };
                total_score += match_score;
            },
            Err(_e) => {},
        }
    }
    println!("Total score: {}", total_score);
    Ok(())
}

fn main() -> io::Result<()> {
    let file = File::open("/home/d/Development/AOC-2022/aoc202202/src/input.txt")?;
    let reader = BufReader::new(file);
    let lookup_table:HashMap<String, u8> = HashMap::from([
        ("A X".to_string(), 3),
        ("C X".to_string(), 2),
        ("B X".to_string(), 1),
        ("A Z".to_string(), 2),
        ("B Z".to_string(), 3),
        ("C Z".to_string(), 1),
        ("A Y".to_string(), 1),
        ("B Y".to_string(), 2),
        ("C Y".to_string(), 3)
    ]);
    let mut total_score: i32 = 0;

    for line in reader.lines() {
        match line {
            Ok(s) => {
                let move_p1 = i32::from(s.as_bytes()[0] - 64);
                let move_p2 = i32::from(lookup_table[&s]);

                let move_sub = move_p1 - move_p2;

                let match_score = match move_sub {
                    -2 => move_p2,
                    -1 => 6 + move_p2,
                    0 =>  3 + move_p2,
                    1 => move_p2,
                    2 => 6 + move_p2,
                    _ => 0,
                };
                total_score += match_score;
            },
            Err(_e) => {},
        }
    }
    println!("Total score: {}", total_score);
    Ok(())
}
//Rock = A = 1
//Paper = B = 2
//Scissors C = 3

// X - Need to lose
// Y - Need to tie
// Z - Need to win

//Lost = 0
//Won = 6
//Draw = 3

// A Z Rock,     Win  = Paper    1  1 = 2
// B Z Paper,    Win  = Scissors 2  1 = 3
// C Z Scissors, Win  = Rock     3 -2 = 1

// A X Rock,     Lose = Scissors 1  2 = 3
// C X Scissors, Lose = Paper    3 -1 = 2
// B X Paper,    Lose = Rock     2 -1 = 1

// A Y Rock,     Tie  = Rock     1  0 = 1
// B Y Paper,    Tie  = Paper    2  0 = 2
// C Y Scissors, Tie  = Scissors 3  0 = 3