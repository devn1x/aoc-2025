use std::fs;

//const INPUT_PATH: &str = "input/06/example.txt";
const INPUT_PATH: &str = "input/06/input.txt";

#[derive(Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operator: Operator
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
    let line_operators = *lines.last().unwrap();
    let mut problems: Vec<Problem> = vec![];

    for o in line_operators.split_ascii_whitespace() {
        let operator = match o {
            "+" => {
                Operator::Add
            }
            "*" => {
                Operator::Multiply
            }
            _ => {
                panic!("This shouldn't happen.");
            }
        };
        let problem = Problem {
            numbers: vec![],
            operator: operator
        };
        problems.push(problem);
    }

    for line_index in 0..lines.len()-1 {
        let line = lines[line_index];
        let split_line: Vec<_> = line.split_ascii_whitespace().collect();
        for number_index in 0..split_line.len() {
            let number: u64 = split_line[number_index].parse().unwrap();
            problems[number_index].numbers.push(number);
        }
        println!("{}: {:?}", line_index, split_line);
    }

    println!("{:?}", problems);

    // Processing
    let mut total = 0;

    for problem in problems {
        total += problem.solve();
        println!("{:?} => {}", problem, problem.solve());
    }


    println!();
    println!("Total: {}", total);
}