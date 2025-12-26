use std::env;
mod solutions;
mod selector;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    if args.len() >= 2 {
        let argument = &args[1];
        let (day, part) = argument.split_once(".").unwrap();
        let day: u8 = day.parse().unwrap();
        let part: u8 = part.parse().unwrap();

        let puzzle = selector::run(Some((day, part)));

        if puzzle.is_some() {
            let puzzle = puzzle.unwrap();
            println!("{:?}", puzzle);
            (puzzle.main)();
        } else {
            println!("Could not find a puzzle matching the parameter '{}'. Please check it exists.", argument);
        }
    } else {
        let puzzle = selector::run(None);

        if puzzle.is_some() {
            let puzzle = puzzle.unwrap();
            println!("{:?}", puzzle);
            (puzzle.main)();
        }
    }
}