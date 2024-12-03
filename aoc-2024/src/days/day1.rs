use std::io::{self, BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn main() -> io::Result<()>{
    let reader = BufReader::new(File::open("res/input.txt")?);

    let (mut first_list, mut second_list): (Vec<i32>, Vec<i32>) = reader.lines()
        .map(|line| {
            let line = line?;
            let numbers: Vec<i32> = line.split_whitespace()
                .map(|num| num.parse().expect("Failed to parse first number"))
                .collect();
            Ok((numbers[0], numbers[1]))
        })
        .collect::<Result<Vec<_>, io::Error>>()?
        .into_iter()
        .unzip();

    let distance = get_distance(&mut first_list, &mut second_list);
    println!("Distance: {distance}");

    let similarity = get_similarity(&mut first_list, &mut second_list);
    println!("Similarity: {similarity}");

    Ok(())
}

fn get_distance(first_list: &[i32], second_list: &[i32]) -> i32 {
    let mut sorted_first = first_list.to_vec();
    let mut sorted_second = second_list.to_vec();

    sorted_first.sort();
    sorted_second.sort();

    sorted_first
        .iter()
        .zip(sorted_second.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn get_similarity(first_list: &[i32], second_list: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &num in second_list {
        *counts.entry(num).or_insert(0) += 1;
    }
    
    first_list
        .iter()
        .map(|&num| num * counts.get(&num).unwrap_or(&0))
        .sum()
}
