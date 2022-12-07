use std::{io::{self, BufReader, BufRead}, fs::{File}, collections::{VecDeque, HashMap}};

fn instruction_executor(queue: &mut HashMap<u32, VecDeque<char>>, action_line: &str) {
    let instruction: String = action_line.chars().filter(|c| c.is_numeric() || c.is_whitespace()).collect();
    let ins_split:Vec<&str> = instruction.split_whitespace().collect();

    let num_moved = ins_split[0].parse::<u32>().unwrap();
    let from = ins_split[1].parse::<u32>().unwrap();
    let to = ins_split[2].parse::<u32>().unwrap();

    for _n in 0..num_moved{
        let item = queue.get_mut(&from).unwrap().pop_front().unwrap();
        queue.get_mut(&to).unwrap().push_front(item);
    }
}

fn instruction_executor_v2(queue: &mut HashMap<u32, VecDeque<char>>, action_line: &str) {
    let instruction: String = action_line.chars().filter(|c| c.is_numeric() || c.is_whitespace()).collect();
    let ins_split:Vec<&str> = instruction.split_whitespace().collect();

    let num_moved = ins_split[0].parse::<u32>().unwrap();
    let from = ins_split[1].parse::<u32>().unwrap();
    let to = ins_split[2].parse::<u32>().unwrap();

    let mut vec_tmp:VecDeque<char> = VecDeque::new();

    for _n in 0..num_moved{
        let item = queue.get_mut(&from).unwrap().pop_front().unwrap();
        vec_tmp.push_front(item);
    }

    for item in vec_tmp{
        queue.get_mut(&to).unwrap().insert(0, item);
    }
}

fn main() -> io::Result<()> {
    let file = File::open("/home/d/Development/AOC-2022/aoc202205/src/input.txt")?;
    let reader = BufReader::new(file);
    let mut crate_stack: HashMap<u32, VecDeque<char>> = HashMap::new();
    let instructions_ln = 0;
    let mut width = 0;
    let mut line_idx = 1;
    let mut crate_height = 0;
    let mut current_line = 0;

    let lines: Vec<_> = reader.lines().collect();

    for line in &lines {
        let lf = line.as_ref().unwrap().trim();
        if lf.starts_with("1"){
            width = lf.split_ascii_whitespace().last().unwrap().parse::<u32>().unwrap();
            break;
        }
        crate_height+=1;
    }
    
    for n in 1..width+1 {
        let mut v :VecDeque<char> = VecDeque::new();

        for line in &lines{
            current_line+=1;
            if current_line > crate_height {
                break
            }
            
            let letter = line.as_ref().unwrap().chars().nth(line_idx).unwrap();
            if letter.is_alphabetic(){
                v.push_back(letter);
            }
        }
        crate_stack.insert(n, v);
        current_line = 0;
        line_idx += 4;
    }

    for line in &lines[instructions_ln..] {
        let ln = line.as_deref().unwrap_or_default();
        if ln.contains("move"){
            instruction_executor_v2(&mut crate_stack, ln);
        }
    }

    for n in 1..width+1{
        let res = crate_stack.get_mut(&n).unwrap();
        print!("{}", res.pop_front().unwrap());
    }

    Ok(())
}
