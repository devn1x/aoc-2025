use std::{fs, path::Path};

//const INPUT_PATH: &str = "input/01/example.txt";
const INPUT_PATH: &str = "input/01/input.txt";
const INITIAL_VALUE: i32 = 50;

pub fn main() {
    let path = Path::new(INPUT_PATH);
    let content = fs::read_to_string(path).unwrap();
    
    let mut value = INITIAL_VALUE;
    let mut total = 0;
    for line in content.lines() {
        let direction = line.split_at(1).0;
        let distance: i32 = line.split_at(1).1.parse().unwrap();
        let next_value = match direction {
            "L" => value - distance,
            "R" => value + distance,
            _ => panic!("Invalid direction found: {}", direction)
        };

        println!("Line: {}", line);
        
        match direction {
            "L" => {
                //println!("    {} -> {}", value, next_value);
                if next_value <= 0 {
                    let rounds = (value - 1).div_euclid(100) - (next_value - 1).div_euclid(100); // And then this fucking line
                    if rounds > 0 {
                        println!("    => {} times", rounds);
                    }
                    total += rounds;
                }
            },
            "R" => {
                //println!("    {} -> {}", value, next_value);
                if next_value >= 100 {
                    let rounds = next_value.div_euclid(100);
                    if rounds > 0 {
                        println!("    => {} times", rounds);
                    }
                    total += rounds;
                }
            },
            _ => panic!("Invalid direction found: {}", direction)
        }

        value = next_value.rem_euclid(100);
        //println!("    new: {}", value);

    }

    println!("Final value: {:?}", value);
    println!("Counted: {}", total);
}