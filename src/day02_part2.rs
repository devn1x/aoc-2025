use std::{fs, path::Path};

const INPUT_PATH: &str = "input/02/input.txt";
//const INPUT_PATH: &str = "input/02/example.txt";

pub fn main() {
    // TEST
    /* println!("DEBUG: {}", validate(2121212121));
    println!("DEBUG: {}", validate(222220));
    return; */
    // TEST

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
            println!("    id invalid: {}", i);
            total += i;
        }
    }
    return total;
}

fn validate(id: u64) -> bool {
    let id = id.to_string();
    let length = id.len();
    
    let chunk_sizes = chunk_sizes(length);
    for size in &chunk_sizes {
        let chunks: Vec<&str> = id.as_bytes().chunks(*size as usize).map(str::from_utf8).collect::<Result<Vec<&str>, _>>().unwrap();
        for index in 1..chunks.len() {
            let first = chunks[0];
            if first != chunks[index] {
                break;
            }
            // If it's the last iteration
            if index == chunks.len() - 1 {
                return false;
            }
        }
        //println!("chunk size {} for value {}: {:?}", size, id, chunks);
    }

    return true;
}

fn chunk_sizes(length: usize) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    for i in 1..length {
        if length % i == 0 {
            //println!("test {}/{}: {}", i, length, length % i);
            result.push(i as u32);
        }
    };
    result
}