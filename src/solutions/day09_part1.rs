use std::fs;

//const INPUT_PATH: &str = "input/09/example.txt";
const INPUT_PATH: &str = "input/09/input.txt";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64
}

#[derive(Debug, PartialEq, Eq)]
struct FloorMap {
    points: Vec<Point>
}

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();

    let mut map: FloorMap = FloorMap { points: vec![] };
    for line in content.lines() {
        //println!("Line: {}", line);
        let parts: Vec<i64> = line.split(",").map(|e| {
            e.parse().unwrap()
        }).collect();
        let point = Point {
            x: parts[0],
            y: parts[1]
        };
        map.points.push(point);
    }

    let total = solve(&mut map);

    //println!("{:?}", map.points);

    println!();
    println!("Total: {}", total);
}

fn solve(map: &mut FloorMap) -> i64 {
    let mut combinations: Vec<(Point, Point)> = vec![];
    for i in 0..map.points.len() {
        for j in (i+1)..map.points.len() {
            if i != j {
                combinations.push((map.points[i], map.points[j]));
            }
        }
    }
    combinations.sort_by_cached_key(|p| {
        if p.0.x == 11 && p.1.x == 2 {
            println!("Considering pair: {:?} and {:?} => {}", p.0, p.1, area(p.0, p.1));
        }
        area(p.0, p.1)
    });

    //println!("Combinations by area: {:#?}", combinations);
    let (p1, p2) = combinations.last().unwrap().clone();
    println!("Largest area between points {:?} and {:?}", p1, p2);
    area(p1, p2)
}

fn area(p1: Point, p2: Point) -> i64 {
    let width = (p2.x - p1.x).abs() + 1;
    let height = (p2.y - p1.y).abs() + 1;
    width * height
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