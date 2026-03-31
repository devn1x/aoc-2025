use std::{
    fmt::Display,
    fs,
    io::{Write, stdout},
};

//const INPUT_PATH: &str = "input/09/example.txt";
const INPUT_PATH: &str = "input/09/input.txt";

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

const COLOR_RED: Color = Color { r: 255, g: 0, b: 0 };
const COLOR_GREEN: Color = Color { r: 0, g: 255, b: 0 };
const COLOR_WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

#[derive(Debug, PartialEq, Eq)]
struct FloorMap {
    red: Vec<Point>,
    green: Vec<Point>,
    x_map: Vec<i64>,
    y_map: Vec<i64>,
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

        const SCALE: i64 = 200;

        let image_width = width / SCALE as usize + 10;
        let image_height = height as usize / SCALE as usize + 10;

        let mut pixels = vec![vec![COLOR_WHITE; image_width]; image_height];

        for green in &self.green {
            let gx = green.x - min_x;
            let gy = green.y - min_y;

            let x_idx = (gx / SCALE) as usize;
            let y_idx = (gy / SCALE) as usize;
            pixels[y_idx][x_idx] = COLOR_GREEN;
        }

        for red in &self.red {
            let rx = red.x - min_x;
            let ry = red.y - min_y;

            let x_idx = (rx / SCALE) as usize;
            let y_idx = (ry / SCALE) as usize;
            pixels[y_idx][x_idx] = COLOR_RED;
        }

        writeln!(&file, "P3")?;
        writeln!(&file, "{} {}", image_width, image_height)?;
        writeln!(&file, "255")?;

        for y in 0..image_height {
            for x in 0..image_width {
                let pixel = pixels[y][x];
                write!(&file, "{} {} {} ", pixel.r, pixel.g, pixel.b)?;
            }
            writeln!(&file)?;
        }
        Ok(())
    }

    fn save_ppm_compressed(self: &Self, path: &str) -> Result<(), std::io::Error> {
        let file = fs::File::create(path).unwrap();
        const PADDING: i64 = 0;

        let (min_x, max_x, min_y, max_y) = self.bounding_box();
        let min_x = min_x - PADDING;
        let max_x = max_x + PADDING;
        let min_y = min_y - PADDING;
        let max_y = max_y + PADDING;

        let width = (max_x - min_x + 1) as usize;
        let height = (max_y - min_y + 1) as usize;

        let mut pixels = vec![vec![COLOR_WHITE; width]; height];

        for green in &self.green {
            let gx = (green.x - min_x) as usize;
            let gy = (green.y - min_y) as usize;
            pixels[gy][gx] = COLOR_GREEN;
        }

        for red in &self.red {
            let rx = (red.x - min_x) as usize;
            let ry = (red.y - min_y) as usize;
            pixels[ry][rx] = COLOR_RED;
        }

        writeln!(&file, "P3")?;
        writeln!(&file, "{} {}", width, height)?;
        writeln!(&file, "255")?;

        for y in 0..height {
            for x in 0..width {
                let pixel = pixels[y][x];
                write!(&file, "{} {} {} ", pixel.r, pixel.g, pixel.b)?;
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
        x_map: vec![],
        y_map: vec![],
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

    map.save_ppm("day09_part2_initial.ppm").unwrap();
    println!("Initial map saved");

    compress_coordinates(&mut map);
    println!("Coordinates compressed");

    green_tiles_border(&mut map);
    println!("Green borders drawn");

    map.save_ppm_compressed("day09_part2_compressed.ppm")
        .unwrap();
    println!("Compressed map saved");

    let total = solve(&mut map);

    println!();
    println!("Total: {}", total);
}

fn solve(map: &FloorMap) -> i64 {
    let length = map.red.len();
    let mid_top = length / 2;
    let mid_bot = mid_top + 1;

    let mut max_area: i64 = 0;

    // ################## TOP HALF ##################
    let corner = map.red[mid_top];

    let mut lo = 0;
    let mut hi = mid_top / 2;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if map.red[mid].x >= corner.x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    let y_bound = map.red[lo].y;

    let mut j = mid_top - 1;
    let mut max_x = 0;
    while map.red[j].y <= y_bound {
        if map.red[j].x >= max_x {
            max_x = map.red[j].x;
            let orig_corner = decompress_point(map, corner);
            let orig_other = decompress_point(map, map.red[j]);
            let area = (orig_corner.x - orig_other.x + 1) * (orig_other.y - orig_corner.y + 1);
            max_area = max_area.max(area);
        }
        if j == 0 { break; }
        j -= 1;
    }

    // ################# BOTTOM HALF ##################
    let corner = map.red[mid_bot];

    lo = (length + mid_bot) / 2;
    hi = length - 1;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if map.red[mid].x >= corner.x {
            hi = mid - 1;
        } else {
            lo = mid;
        }
    }
    let y_bound = map.red[lo].y;

    j = mid_bot + 1;
    max_x = 0;
    while j < length && map.red[j].y >= y_bound {
        if map.red[j].x >= max_x {
            max_x = map.red[j].x;
            let orig_corner = decompress_point(map, corner);
            let orig_other = decompress_point(map, map.red[j]);
            let area = (orig_corner.x - orig_other.x + 1) * (orig_corner.y - orig_other.y + 1);
            max_area = max_area.max(area);
        }
        j += 1;
    }

    max_area
}

fn area(p1: Point, p2: Point) -> i64 {
    let width = (p2.x - p1.x).abs() + 1;
    let height = (p2.y - p1.y).abs() + 1;
    width * height
}

fn compress_coordinates(map: &mut FloorMap) {
    use std::collections::BTreeSet;

    // Sammle alle einzigartigen x- und y-Koordinaten
    let mut x_coords: BTreeSet<i64> = BTreeSet::new();
    let mut y_coords: BTreeSet<i64> = BTreeSet::new();

    for point in &map.red {
        x_coords.insert(point.x);
        y_coords.insert(point.y);
    }

    for point in &map.green {
        x_coords.insert(point.x);
        y_coords.insert(point.y);
    }

    // Erstelle Mapping von alter zu neuer Koordinate
    map.x_map = x_coords.into_iter().collect();
    map.y_map = y_coords.into_iter().collect();

    // Komprimiere alle Punkte
    for point in &mut map.red {
        point.x = map.x_map.binary_search(&point.x).unwrap() as i64;
        point.y = map.y_map.binary_search(&point.y).unwrap() as i64;
    }

    for point in &mut map.green {
        point.x = map.x_map.binary_search(&point.x).unwrap() as i64;
        point.y = map.y_map.binary_search(&point.y).unwrap() as i64;
    }

    println!(
        "Compressed from coordinate space to {}x{}",
        map.x_map.len(),
        map.y_map.len()
    );
}

fn decompress_point(map: &FloorMap, point: Point) -> Point {
    Point {
        x: map.x_map[point.x as usize],
        y: map.y_map[point.y as usize],
    }
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

fn is_inside(map: &FloorMap, point: Point) -> bool {
    let mut inside = false;
    let n = map.red.len();
    
    for i in 0..n {
        let p1 = map.red[i];
        let p2 = map.red[(i + 1) % n];
        
        if ((p1.y > point.y) != (p2.y > point.y)) {
            let x_intersect = (p2.x - p1.x) * (point.y - p1.y) / (p2.y - p1.y) + p1.x;
            
            if point.x < x_intersect {
                inside = !inside;
            }
        }
    }
    
    inside
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

    #[test]
    fn test_example() {
        let mut map = FloorMap {
            red: vec![
                Point { x: 7, y: 1 },
                Point { x: 11, y: 1 },
                Point { x: 11, y: 7 },
                Point { x: 9, y: 7 },
                Point { x: 9, y: 5 },
                Point { x: 2, y: 5 },
                Point { x: 2, y: 3 },
                Point { x: 7, y: 3 },
            ],
            green: vec![],
            x_map: vec![],
            y_map: vec![],
        };
        green_tiles_border(&mut map);
        compress_coordinates(&mut map);
        assert_eq!(solve(&mut map), 24);
    }
}
