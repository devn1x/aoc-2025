use std::{fs, num};

const INPUT_PATH: &str = "input/08/example.txt";
//const INPUT_PATH: &str = "input/08/input.txt";

#[derive(Debug, PartialEq, Eq)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32
}

#[derive(Debug, PartialEq, Eq)]
struct Circuit {
    junctions: Vec<Vector3>
}

impl Vector3 {
    fn from_string(string: &str) -> Self {
        let parts: Vec<_> = string.split(",").collect();
        Vector3 {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap()
        }
    }
    fn distance(self: &Self, other: &Self) -> i32 {
        let distance = ((other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)).isqrt();
        distance
    }
}

impl Circuit {
    fn append(self: &mut Self, other: &mut Self) {
        self.junctions.append(&mut other.junctions);
    }
}

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<_> = content.lines().collect();

    let mut total = 0;
    let mut circuits: Vec<Circuit> = vec![];
    for line in content.lines() {
        println!("Line: {}", line);
        let junction = Vector3::from_string(line);
        let mut circuit = Circuit { junctions: vec![] };
        circuit.junctions.push(junction);
        circuits.push(circuit);
    }
    println!("{:#?}", circuits);

    for iteration in 0..10 {
        let shortest_connection = shortest_distance(&circuits);
        let circuit = circuit_by_junction(&shortest_connection.0);
        let circuit2 = circuit_by_junction(&shortest_connection.1);
        circuit.append(circuit2);
        //circuits.retain(|c| c != circuit2);
    }

    println!();
    println!("Total: {}", total);
}

fn shortest_distance(circuits: &Vec<Circuit>) -> (Vector3, Vector3) {
    todo!()
}

fn circuit_by_junction(junction: &Vector3) -> &'static mut Circuit {
    todo!()
}
