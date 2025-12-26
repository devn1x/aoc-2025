use std::{collections::HashMap, fs};

//const INPUT_PATH: &str = "input/05/example.txt";
const INPUT_PATH: &str = "input/05/input.txt";

struct FreshRange {
    start: u64,
    end: u64
}

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();

    // Parsing
    let mut ranges: HashMap<(u64, u64), FreshRange> = HashMap::new();
    let mut ingredients: Vec<u64> = vec![];
    for line in content.lines() {
        if line.trim() == "" {
            continue;
        }
        //println!("{}", line);
        if line.contains("-") {
            let (left, right) = line.split_once("-").unwrap();
            let range = FreshRange {
                start: left.parse().unwrap(),
                end: right.parse().unwrap()
            };
            if !ranges.contains_key(&(range.start, range.end)) {
                ranges.insert((range.start, range.end), range);
            }
        } else {
            let ingredient: u64 = line.parse().unwrap();
            ingredients.push(ingredient);
        }
    }

    // Processing
    let mut total = 0;
    for ingredient in ingredients {
        if in_range(&ranges, ingredient) {
            println!("{} is in range", ingredient);
            total += 1;
        }
    }

    println!();
    println!("Total: {}", total);
}

fn in_range(ranges: &HashMap<(u64, u64), FreshRange>, ingredient: u64) -> bool {
    let filtered_ranges: Vec<(&(u64, u64), &FreshRange)> = ranges.iter().filter(|kv| {
        let key = kv.0;
        key.0 <= ingredient && key.1 >= ingredient
    }).collect();
    filtered_ranges.len() >= 1
}