use std::{fs, path::Path};

const INPUT_PATH: &str = "input/01/input_1.txt";
const INITIAL_VALUE: i32 = 50;

#[derive(Debug)]
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
    fn apply(self: &mut Self, dial_move: &DialMove) -> bool {
        self.value += match dial_move.direction {
            DialMoveDirection::L => {
                -1 * dial_move.distance
            },
            DialMoveDirection::R => {
                dial_move.distance
            }
        };

        self.value = self.value % 100;
        if self.value < 0 {
            self.value += 100;
        }
        self.value == 0
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
        if state.apply(&dial_move) {
            total += 1;
        }
        println!("Line: {:?} \t=> {:?}", dial_move, state.value);
    }

    println!("Final state: {:?}", state);
    println!("Counted: {}", total);
}