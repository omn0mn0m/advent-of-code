use std::fs;
use regex::Regex;

pub fn main() -> std::io::Result<()>{
    let corrupted_memory = fs::read_to_string("res/input3.txt")?;
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();

    let mut sum = 0;
    
    for result in re.find_iter(&corrupted_memory) {
        sum += mul(result);
    }

    println!("Sum: {sum}");

    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();

    sum = 0; // reset for part 2
    let mut enabled = true;

    for result in re.find_iter(&corrupted_memory) {
        if result.as_str() == "do()" {
            enabled = true;
        } else if result.as_str() == "don't()" {
            enabled = false;
        } else {
            if enabled {
                sum += mul(result);
            }
        }
    }

    println!("Better Sum: {sum}");

    Ok(())
}

fn mul(instruction: regex::Match) -> i32 {
    let inst_str: &str = instruction.as_str();
    
    let inside = &inst_str[4..inst_str.len() - 1];
    let parts: Vec<&str> = inside.split(',').collect();

    let first: i32 = parts[0].parse().unwrap();
    let second: i32 = parts[1].parse().unwrap();

    return first * second;
}
