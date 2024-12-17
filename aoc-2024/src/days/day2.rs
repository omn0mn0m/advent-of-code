use std::io::{self, BufRead, BufReader};
use std::fs::File;

pub fn main() -> io::Result<()>{
    let reader = BufReader::new(File::open("res/input.txt")?);

    // Parse file into reports with levels
    let mut reports = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        reports.push(numbers);
    }

    let mut safe_count = 0;
    
    for report in &reports {
        if is_safe(report) {
            safe_count += 1;
        }
    }

    println!("Safe Count: {safe_count}");

    let mut dampened_safe_count = 0;

    for report in &reports {
        let mut dampened_is_safe = false;

        for i in 0..report.len() {
            let dampened_report: Vec<i32> = remove_at_index(&report, i);

            if is_safe(&dampened_report) {
                dampened_is_safe = true;
            }
        }

        if dampened_is_safe {
            dampened_safe_count += 1;
        }
    }

    println!("Dampened Safe Count: {dampened_safe_count}");

    
    Ok(())
}

fn remove_at_index(vec: &Vec<i32>, i: usize) -> Vec<i32> {
    vec.iter()
        .enumerate()
        .filter(|&(index, _)| index != i)
        .map(|(_, value)| *value)
        .collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
    let is_increasing = report[0] < report[1];
    let mut is_safe = true;
    
    for i in 1..report.len() {
        if is_increasing == (report[i-1] < report[i]) {
            let diff = (report[i] - report[i-1]).abs();
            if diff < 1 || diff > 3 {
                is_safe = false;
                break; // steps too large or too small
            }
        } else {
            is_safe = false;
            break; // not sequential
        }
    }

    return is_safe;
}
