use std::env;

mod day01_part1;
mod day01_part2;
mod day02_part1;
mod day02_part2;
mod day03_part1;
mod day03_part2;
mod day04_part1;
mod day04_part2;
mod day05_part1;
mod day05_part2;
mod day06_part1;
mod day06_part2;
mod day07_part1;

#[derive(Debug)]
struct Puzzle {
    day: u8,
    part: u8,
    main: fn()
}

fn main() {
    let puzzles: Vec<Puzzle> = vec![
        Puzzle { day: 1, part: 1, main: day01_part1::main },
        Puzzle { day: 1, part: 2, main: day01_part2::main },
        Puzzle { day: 2, part: 1, main: day02_part1::main },
        Puzzle { day: 2, part: 2, main: day02_part2::main },
        Puzzle { day: 3, part: 1, main: day03_part1::main },
        Puzzle { day: 3, part: 2, main: day03_part2::main },
        Puzzle { day: 4, part: 1, main: day04_part1::main },
        Puzzle { day: 4, part: 2, main: day04_part2::main },
        Puzzle { day: 5, part: 1, main: day05_part1::main },
        Puzzle { day: 5, part: 2, main: day05_part2::main },
        Puzzle { day: 6, part: 1, main: day06_part1::main },
        Puzzle { day: 6, part: 2, main: day06_part2::main },
        Puzzle { day: 7, part: 1, main: day07_part1::main }
    ];

    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    if args.len() >= 2 {
        let argument = &args[1];
        let (day, part) = argument.split_once(".").unwrap();
        let day: u8 = day.parse().unwrap();
        let part: u8 = part.parse().unwrap();

        let puzzle: Vec<&Puzzle> = puzzles.iter().filter(|e| {
            e.day == day && e.part == part
        }).collect();
        if puzzle.len() == 0 {
            return;
        }
        let puzzle = puzzle[0];

        println!("{:?}", puzzle);
        (puzzle.main)();
    } else {
        let latest = puzzles.last();
        if latest.is_some() {
            let latest = latest.unwrap();
            (latest.main)();
        }
    }
}