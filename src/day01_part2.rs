use std::{fs, path::Path};

const INPUT_PATH: &str = "input/01/input.txt";
const INITIAL_VALUE: i32 = 50;

#[derive(Debug, Copy, Clone)]
enum DialMoveDirection {
    L,
    R
}

#[derive(Debug)]
struct DialMove {
    direction: DialMoveDirection,
    distance: i32
}

impl DialMove {
    fn from_str(line: &str) -> Self {
        let (direction, distance) = line.split_at(1);
        let direction = match direction {
            "L" => DialMoveDirection::L,
            "R" => DialMoveDirection::R,
            _ => panic!("Invalid direction")
        };
        DialMove { direction: direction, distance: distance.parse().unwrap() }
    }
}

#[derive(Debug)]
struct StateMachine {
    value: i32
}

impl StateMachine {
    fn apply(self: &mut Self, dial_move: &DialMove) -> i32 {
        let previous = self.value;
        let direction = match &dial_move.direction {
            DialMoveDirection::L => -1,
            DialMoveDirection::R => 1,
        };
        let total_distance = dial_move.distance * direction;
        self.value += total_distance;

        println!("Pre-normalize: {}", self.value);

        //self.value = self.value % 100;
        let mut total = 0;
        while self.value < 0 || self.value >= 100 {
            if self.value < 0 {
                self.value += 100;
            } else if self.value >= 100 {
                self.value -= 100;
            }
        }

        println!("{} -> {} = {} ({})", previous, self.value, self.value - previous, total_distance);
        if previous + total_distance >= 100 {
            total += (f64::from(previous + total_distance) / f64::from(100)).floor() as i32;
        } else if previous + total_distance < 0 {
            total += (f64::from(previous + total_distance) / f64::from(100)).floor() as i32 * -1;
        }

        return total;
    }
}

pub fn main() {
    let path = Path::new(INPUT_PATH);
    let mut state = StateMachine {
        value: INITIAL_VALUE
    };

    let content = fs::read_to_string(path).unwrap();
    
    let mut total = 0;
    for line in content.lines() {
        let dial_move = DialMove::from_str(line);
        let zeroes = state.apply(&dial_move);
        total += zeroes;
        println!("Line: {:?} \t=> {:?} \t({:?})", dial_move, state.value, zeroes);
    }

    println!("Final state: {:?}", state);
    println!("Counted: {}", total);

    println!("----- L953 -----");
    let test = DialMove::from_str("L953");
    let mut state = StateMachine { value: INITIAL_VALUE };
    let zeroes = state.apply(&test);
    println!("Test L953: {:?} => {:?} ({})", test, state.value, zeroes);
    println!("----------------");
    println!("{}", (f64::from(953) / f64::from(100)).floor());
}