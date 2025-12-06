use std::{cmp, collections::HashMap, fs};

//const INPUT_PATH: &str = "input/05/example.txt";
const INPUT_PATH: &str = "input/05/input.txt";

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
struct FreshRange {
    start: u64,
    end: u64
}

impl Ord for FreshRange {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.start < other.start {
            cmp::Ordering::Less
        } else if self.start > other.start {
            cmp::Ordering::Greater
        } else {
            cmp::Ordering::Equal
        }
    }
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
    let merged_ranges = merge_ranges(ranges);
    for range in merged_ranges {
        total += range.end - range.start + 1;
    }


    println!();
    println!("Total: {}", total);
}

fn merge_ranges(ranges: HashMap<(u64, u64), FreshRange>) -> Vec<FreshRange> {
    let mut ranges: Vec<_> = ranges.iter().map(|e| {
        e.1
    }).collect();
    ranges.sort();

    //println!("{:?}", ranges);

    let mut result: Vec<FreshRange> = vec![];
    result.push(*ranges[0]);

    for curr in ranges.iter().skip(1) {
        let last_end = result.last().unwrap().end;

        // If current interval overlaps with the last merged interval, merge them
        if curr.start <= last_end {
            let last = result.last_mut().unwrap();
            last.end = cmp::max(last.end, curr.end);
        } else {
            result.push(**curr);
        }
    }

    return result;
}

#[test]
fn test_examples() {
    // assert_eq!(987654321111, max_joltage("987654321111111", 12));
    // assert_eq!(888911112111, max_joltage("811111111111119", 12));
    // assert_eq!(434234234278, max_joltage("234234234234278", 12));
    // assert_eq!(888911112111, max_joltage("818181911112111", 12));
}