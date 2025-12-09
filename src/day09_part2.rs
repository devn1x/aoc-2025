use std::{
    fmt::Display,
    fs,
    io::{Write, stdout},
};

//const INPUT_PATH: &str = "input/09/example.txt";
const INPUT_PATH: &str = "input/09/input.txt";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct FloorMap {
    red: Vec<Point>,
    green: Vec<Point>
}

impl FloorMap {
    fn bounding_box(self: &Self) -> (i64, i64, i64, i64) {
        let min_x = self.red.iter().map(|p| p.x).min().unwrap();
        let max_x = self.red.iter().map(|p| p.x).max().unwrap();
        let min_y = self.red.iter().map(|p| p.y).min().unwrap();
        let max_y = self.red.iter().map(|p| p.y).max().unwrap();

        (min_x, max_x, min_y, max_y)
    }

    fn save_ppm(self: &Self, path: &str) -> Result<(), std::io::Error> {
        let file = fs::File::create(path).unwrap();
        const PADDING: i64 = 1;

        let (min_x, max_x, min_y, max_y) = self.bounding_box();
        let min_x = min_x - PADDING;
        let max_x = max_x + PADDING;
        let min_y = min_y - PADDING;
        let max_y = max_y + PADDING;

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;

        const SCALE: i64 = 1000;

        writeln!(&file, "P3")?;
        writeln!(&file, "{} {}", width/SCALE as usize, height/SCALE as usize)?;
        writeln!(&file, "255")?;

        for y in 0..=height {
            for x in 0..=width {
                let is_red = self.red.iter().any(|g| {
                    let gx = g.x - min_x;
                    let gy = g.y - min_y;
                    (gx >= (x as i64)*SCALE && gx < ((x as i64)+SCALE-1)*SCALE) &&
                    (gy >= (y as i64)*SCALE && gy < ((y as i64)+SCALE-1)*SCALE)
                });
                let is_green = self.green.iter().any(|g| {
                    let gx = g.x - min_x;
                    let gy = g.y - min_y;
                    (gx >= (x as i64)*SCALE && gx < ((x as i64)+SCALE-1)*SCALE) &&
                    (gy >= (y as i64)*SCALE && gy < ((y as i64)+SCALE-1)*SCALE)
                });

                if is_red {
                    write!(&file, "255 0 0    ")?;
                } else if is_green {
                    write!(&file, "0 255 0    ")?;
                } else {
                    write!(&file, "255 255 255    ")?;
                }
            }
            writeln!(&file)?;
        }
        Ok(())
    }
}

impl Display for FloorMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PADDING: i64 = 1;

        let (min_x, max_x, min_y, max_y) = self.bounding_box();
        let min_x = min_x - PADDING;
        let max_x = max_x + PADDING;
        let min_y = min_y - PADDING;
        let max_y = max_y + PADDING;

        println!("Map from ({},{}) to ({},{})", min_x, min_y, max_x, max_y);
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let point = Point { x, y };
                if self.red.contains(&point) {
                    write!(f, "# ")?;
                } else if self.green.contains(&point) {
                    write!(f, "X ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();

    let mut map: FloorMap = FloorMap {
        red: vec![],
        green: vec![],
    };
    for line in content.lines() {
        //println!("Line: {}", line);
        let parts: Vec<i64> = line.split(",").map(|e| e.parse().unwrap()).collect();
        let point = Point {
            x: parts[0],
            y: parts[1],
        };
        map.red.push(point);
    }
    println!("Red tiles loaded: {}", map.red.len());

    green_tiles_border(&mut map);
    println!("Green borders drawn");

    map.save_ppm("day09_part2_initial.ppm");

    //println!("{}", map);

    //fill_green_tiles(&mut map);

    //println!("{}", map);

    let total = solve(&mut map);

    println!();
    println!("Total: {}", total);
}

fn solve(map: &mut FloorMap) -> i64 {
    let mut combinations: Vec<(Point, Point)> = vec![];
    for i in 0..map.red.len() {
        for j in (i + 1)..map.red.len() {
            if i != j {
                combinations.push((map.red[i], map.red[j]));
            }
        }
    }
    println!("Total combinations: {}", combinations.len());

    // let mut combinations: Vec<_> = combinations
    //     .iter()
    //     .filter(|combination| validate_combination(map, combination))
    //     .collect();

    combinations.sort_by_cached_key(|p| {
        // if p.0.x == 11 && p.1.x == 2 {
        //     println!(
        //         "Considering pair: {:?} and {:?} => {}",
        //         p.0,
        //         p.1,
        //         area(p.0, p.1)
        //     );
        // }
        // if p.0.x == 9 && p.1.x == 2 {
        //     println!(
        //         "Considering pair: {:?} and {:?} => {}",
        //         p.0,
        //         p.1,
        //         area(p.0, p.1)
        //     );
        // }
        area(p.0, p.1)
    });
    combinations.reverse();
    println!("Combinations sorted: {}", combinations.len());

    //println!("Combinations by area: {:#?}", combinations);
    for (p1, p2) in combinations {
        print!(".");
        stdout().flush().unwrap();
        let mut step_size = 65536 * 8;
        loop {
            //println!("Current step size: {}", step_size);
            let success = validate_combination(map, &(p1, p2), step_size);
            if !success {
                break;
            } else {
                print!("+");
                stdout().flush().unwrap();
            }
            if step_size == 1 {
                println!();
                println!("Largest area between points {:?} and {:?}", p1, p2);
                return area(p1, p2);
            }
            step_size /= 2;
        }
    }
    panic!("No valid combination found");
}

fn area(p1: Point, p2: Point) -> i64 {
    let width = (p2.x - p1.x).abs() + 1;
    let height = (p2.y - p1.y).abs() + 1;
    width * height
}

fn validate_combination(map: &FloorMap, combination: &(Point, Point), step_size: usize) -> bool {
    let (p1, p2) = combination;

    let min_x = p1.x.min(p2.x);
    let max_x = p1.x.max(p2.x);
    let min_y = p1.y.min(p2.y);
    let max_y = p1.y.max(p2.y);

    // for y in min_y..=max_y {
    //     for x in min_x..=max_x {
    //         let point = Point { x, y };
    //         if !map.red.contains(&point) && !map.green.contains(&point) && is_inside(map, point) {
    //             return false;
    //         }
    //     }
    // }

    let anti_p1 = Point{
        x: p1.x,
        y: p2.y
    };
    if !is_inside(map, anti_p1) {
        return false;
    }
    let anti_p2 = Point{
        x: p2.x,
        y: p1.y
    };
    if !is_inside(map, anti_p2) {
        return false;
    }

    for y in [min_y, max_y] {
        for x in (min_x..=max_x).step_by(step_size) {
            let point = Point { x, y };
            //println!("Checking border point {:?}", point);
            if map.green.contains(&point) || map.red.contains(&point) {
                continue;
            }
            if !map.red.contains(&point) && !map.green.contains(&point) && is_inside(map, point) {
                return false;
            }
        }
    }

    for y in (min_y..=max_y).step_by(step_size) {
        for x in [min_x, max_x] {
            let point = Point { x, y };
            //println!("Checking border point {:?}", point);
            if map.red.contains(&point) || map.green.contains(&point) {
                continue;
            }
            if !map.red.contains(&point) && !map.green.contains(&point) && is_inside(map, point) {
                return false;
            }
        }
    }

    true
}

fn green_tiles_border(map: &mut FloorMap) {
    for index in 0..map.red.len() {
        let p1 = &map.red[index];
        let p2 = &map.red[(index + 1).rem_euclid(map.red.len())];
        //println!("Processing edge {}: {:?}", index, p2);

        let from_x = p1.x.min(p2.x);
        let to_x = p1.x.max(p2.x);
        let from_y = p1.y.min(p2.y);
        let to_y = p1.y.max(p2.y);

        for y in from_y..=to_y {
            for x in from_x..=to_x {
                let point = Point { x, y };
                if !map.red.contains(&point) {
                    map.green.push(point);
                }
            }
        }
    }
}

fn fill_green_tiles(map: &mut FloorMap) {
    let (min_x, max_x, min_y, max_y) = map.bounding_box();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let point = Point { x, y };
            if is_inside(map, point) {
                if !map.red.contains(&point) {
                    map.green.push(point);
                }
            }
        }
    }
}

fn is_inside(map: &FloorMap, point: Point) -> bool {
    let has_green_left = map.green.iter().any(|p| p.y == point.y && p.x < point.x);
    if !has_green_left {
        return false;
    }
    let has_green_right = map.green.iter().any(|p| p.y == point.y && p.x > point.x);
    if !has_green_right {
        return false;
    }
    let has_green_above = map.green.iter().any(|p| p.x == point.x && p.y < point.y);
    if !has_green_above {
        return false;
    }
    let has_green_below = map.green.iter().any(|p| p.x == point.x && p.y > point.y);
    if !has_green_below {
        return false;
    }

    has_green_left && has_green_right && has_green_above && has_green_below
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_area() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 4, y: 5 };
        assert_eq!(area(p1, p2), 12);

        let p1 = Point { x: 5, y: 5 };
        let p2 = Point { x: 1, y: 1 };
        assert_eq!(area(p1, p2), 16);
    }
}
