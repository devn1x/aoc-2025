use std::{fs, path::Path};

const INPUT_PATH: &str = "input/02/input.txt";
//const INPUT_PATH: &str = "input/02/example.txt";

pub fn main() {
    let path = Path::new(INPUT_PATH);
    let content = fs::read_to_string(path).unwrap();
    let ranges: Vec<&str> = content.split(",").collect();

    let mut total = 0;
    for range in &ranges {
        let bounds: Vec<&str> = range.split("-").collect();

        let from = bounds[0].parse().unwrap();
        let to = bounds[1].parse().unwrap();
        println!("{} -> {}", from, to);
        total += process_range(from, to);
    }

    println!();
    println!("Total: {}", total);
}

fn process_range(from: u64, to: u64) -> u64 {
    let mut total = 0;
    for i in from..=to {
        let result = validate(i);
        if !result {
            println!("    {}: {}", i, result);
            total += i;
        }
    }
    return total;
}

fn validate(id: u64) -> bool {
    let id = id.to_string();
    let length = id.len();
    let first_half = &id[..length/2];
    let second_half = &id[length/2..];

    if first_half == second_half {
        //println!("{}/{}", first_half, second_half);
        return false;
    }

    return true;
}