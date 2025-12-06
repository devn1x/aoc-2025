use std::fs;

//const INPUT_PATH: &str = "input/06/example.txt";
const INPUT_PATH: &str = "input/06/input.txt";

#[derive(Debug, PartialEq, Eq, Clone)]
enum Operator {
    Add,
    Multiply,
    Unknown,
}

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<u64>,
    operator: Operator,
}
impl Problem {
    fn solve(&self) -> u64 {
        let mut total = 0;
        for n in &self.numbers {
            if self.operator == Operator::Add {
                total += n;
            } else if self.operator == Operator::Multiply {
                if total == 0 {
                    total = *n;
                } else {
                    total *= n;
                }
            }
        }
        total
    }
}

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();

    // Parsing
    let lines: Vec<_> = content.lines().collect();
    let problems: Vec<Problem> = parse(&lines);

    println!("Problems: {:?}", problems);

    // Processing
    let mut total = 0;

    for problem in problems {
        total += problem.solve();
        println!("{:?} => {}", problem, problem.solve());
    }

    println!();
    println!("Total: {}", total);
}

fn parse(lines: &Vec<&str>) -> Vec<Problem> {
    let line_width = lines[0].len();
    //let mut test: Vec<Vec<&str>> = vec![];
    let mut result_string = "".to_string();
    let mut result_part = "".to_string();
    for char_index in (0..line_width).rev() {
        for line_index in 0..lines.len() {
            let char = &lines[line_index][char_index..char_index+1];
            //println!("{}: {:?}", line_index, char);
            if char == "+" || char == "*" {
                result_part += &format!(" {}\n", char);
            } else {
                result_part += char;
            }
        }
        result_string += &result_part;
        result_part = "".to_string();
    }

    let mut result: Vec<Problem> = vec![];
    for line in result_string.lines() {
        let columns: Vec<_> = line.split_ascii_whitespace().rev().collect();
        let mut problem = Problem {
            numbers: vec![],
            operator: Operator::Unknown
        };
        for c in columns {
            println!("{}", c);
            if c == "+" {
                problem.operator = Operator::Add;
            } else if c == "*" {
                problem.operator = Operator::Multiply;
            } else {
                let value: u64 = c.parse().unwrap();
                problem.numbers.push(value);
            }
        }
        result.push(problem);
    }

    println!();
    println!("result: {}", result_string);
    println!("result: {:?}", result);

    for char_index in lines[0].char_indices().rev() {
        for line_index in 0..lines.len() - 1 {
            println!("{}: {:?}", line_index, char_index);
        }
    }
    result
}

#[test]
fn test_examples() {
    // let content = fs::read_to_string(INPUT_PATH).unwrap();
    // let lines: Vec<_> = content.lines().collect();
    // let lines_transformed = transform(&lines);
    // let solution = vec![
    //     "  4 175   8 356",
    //     "431 581 248  24",
    //     "623  32 369   1",
    //     "+   *   +   *",
    // ];
    // assert_eq!(
    //     format!("{:?}", solution),
    //     format!("{:?}", lines_transformed)
    // );
    // assert_eq!(888911112111, max_joltage("811111111111119", 12));
    // assert_eq!(434234234278, max_joltage("234234234234278", 12));
    // assert_eq!(888911112111, max_joltage("818181911112111", 12));
}
