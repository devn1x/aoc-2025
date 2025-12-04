use core::fmt;
use std::{fmt::Display, fs};

//const INPUT_PATH: &str = "input/04/example.txt";
const INPUT_PATH: &str = "input/04/input.txt";

#[derive(Debug, PartialEq, Eq)]
enum MapObject {
    Space,
    Roll(bool)
}

impl MapObject {
    fn from_char(char: char) -> MapObject {
        return match char {
            '.' => {
                MapObject::Space
            }
            '@' => {
                MapObject::Roll(false)
            }
            'x' => {
                MapObject::Roll(true)
            }
            _ => {
                panic!("This is fucked up man... You can't use {} in your map.", char)
            }
        };
    }
}

impl Display for MapObject {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let result = match self {
            MapObject::Space => ".",
            MapObject::Roll(false) => "@",
            MapObject::Roll(true) => "x"
        };
        write!(f, "{}", result).unwrap();
        Ok(())
    }
}

#[derive(Debug)]
struct MapRow {
    cols: Vec<MapObject>
}

impl MapRow {
    fn push(self: &mut Self, object: MapObject) {
        self.cols.push(object);
    }
}

impl Display for MapRow {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for col in &self.cols {
            write!(f, "{}", col).unwrap();
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Map {
    rows: Vec<MapRow>
}

impl Map {
    fn tile(self: &Self, x: isize, y: isize) -> Option<&MapObject> {
        let x: usize = x as usize;
        let y: usize = y as usize;
        self.rows.get(y)?.cols.get(x)
    }

    fn check_tile(self: &Self, x: usize, y: usize) -> Option<bool> {
        let tile = &self.rows[y].cols[x];
        if tile == &MapObject::Space {
            return None;
        }

        let x: isize = x as isize;
        let y: isize = y as isize;
        
        let tile_nw = &self.tile(x - 1, y - 1);
        let tile_nm = &self.tile(x, y - 1);
        let tile_ne = &self.tile(x + 1, y - 1);

        let tile_mw = &self.tile(x - 1, y);
        let tile_me = &self.tile(x + 1, y);

        let tile_sw = &self.tile(x - 1, y + 1);
        let tile_sm = &self.tile(x, y + 1);
        let tile_se = &self.tile(x + 1, y + 1);

        let neighbours = vec![tile_nw, tile_nm, tile_ne, tile_mw, tile_me, tile_sw, tile_sm, tile_se];
        let mut total_neighbours = 0;
        for n in neighbours {
            if n.is_none() {
                continue;
            }
            if matches!(n.unwrap(), MapObject::Roll(_)) {
                total_neighbours += 1;
            }
        }
        return Some(total_neighbours < 4);
    }
}

impl Display for Map {
    fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut result = "".to_string();
        for row in &self.rows {
            result = format!("{}\n{}", result, row);
        }
        write!(f, "{}", result).unwrap();
        Ok(())
    }
}

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();

    // Parsing
    let mut map = Map{
        rows: vec![]
    };
    for line in content.lines() {
        let mut row = MapRow {
            cols: vec![]
        };
        for char in line.chars() {
            row.push(MapObject::from_char(char));
        }
        map.rows.push(row);
    }

    //println!("{}", map);

    // Processing
    let mut total = 0;

    for y in 0..map.rows.len() {
        for x in 0..map.rows[y].cols.len() {
            let accessible = map.check_tile(x, y);
            if accessible.is_some() {
                let accessible = accessible.unwrap();
                map.rows[y].cols[x] = MapObject::Roll(accessible);
                if accessible {
                    total += 1;
                }
            }
        }
    }

    println!("{}", map);

    println!();
    println!("Total: {}", total);
}

#[test]
fn test_examples() {
    //assert_eq!(987654321111, max_joltage("987654321111111", 12));
    // assert_eq!(888911112111, max_joltage("811111111111119", 12));
    // assert_eq!(434234234278, max_joltage("234234234234278", 12));
    // assert_eq!(888911112111, max_joltage("818181911112111", 12));
}