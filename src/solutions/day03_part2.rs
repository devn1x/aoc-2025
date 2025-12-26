use std::fs;

//const INPUT_PATH: &str = "input/03/example.txt";
const INPUT_PATH: &str = "input/03/input.txt";

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();

    let mut total = 0;
    for line in content.lines() {
        println!("--- Line: {}", line);
        let max_joltage = max_joltage(line, 12);
        total += max_joltage;
        println!("  ==> {} ({})", max_joltage, total);
    }

    println!();
    println!("Total: {}", total);
}

fn max_joltage(line: &str, length: usize) -> u64 {
    let batteries: Vec<u8> = line.chars().map(|c| -> u8 {
        c.to_string().parse().unwrap()
    }).collect();
    let mut batteries_sorted = batteries.clone();
    batteries_sorted.sort();
    batteries_sorted.dedup();
    batteries_sorted.reverse();
    
    //let mut max_single_joltage = 0;
    let mut total_joltage = 0;
    for joltage in &batteries_sorted {
        let largest_valid_index = batteries.iter().position(|e| {
            *e == *joltage
        });

        if largest_valid_index.is_none() {
            continue;
        }

        let largest_valid_index = largest_valid_index.unwrap();
        let to_end = batteries.len() - largest_valid_index;
        let remaining_line = line.split_at(largest_valid_index + 1).1;

        if to_end >= length {
            let max_single_joltage = batteries[largest_valid_index];
            //println!("    DEBUG1: biggest valid {:?} (to end: {}; remains: {})", max_single_joltage, to_end, remaining_line);
            if length != 1 {
                let next_digit = max_joltage(remaining_line, length - 1);
                let test: u64 = (max_single_joltage.to_string() + &next_digit.to_string()).parse().unwrap();
                total_joltage = test;
                //println!("    DEBUG3 {}: {} + {} -> {}", length, max_single_joltage, next_digit, test);
                //println!("    DEBUG2 {}: next digit: {} (result: {})", length, next_digit, total_joltage);
            } else {
                //println!("    DEBUG2 {}: last digit: {} (result: {})", length, "-", max_single_joltage);
                total_joltage = max_single_joltage as u64;
            }
            break;
        }
    }

    return total_joltage as u64;
}

#[test]
fn test_examples() {
    assert_eq!(987654321111, max_joltage("987654321111111", 12));
    assert_eq!(888911112111, max_joltage("811111111111119", 12));
    assert_eq!(434234234278, max_joltage("234234234234278", 12));
    assert_eq!(888911112111, max_joltage("818181911112111", 12));
}