use std::{collections::HashMap, fs, hash::{DefaultHasher, Hash, Hasher}};

const INPUT_PATH: &str = "input/08/example.txt";
//const INPUT_PATH: &str = "input/08/input.txt";

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Circuit {
    junctions: Vec<Vector3>
}

#[derive(Debug)]
struct Pair<T> {
    p1: T,
    p2: T
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

impl Hash for Vector3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

impl Hash for Pair<Vector3> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.p2 < self.p1  {
            self.p2.hash(state);
            self.p1.hash(state);
        } else {
            self.p1.hash(state);
            self.p2.hash(state);
        }
    }
}

impl Circuit {
    fn extend(self: &mut Self, other: &Self) {
        self.junctions.extend(other.junctions.clone());
    }
}

fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn main() {
    // let p1 = Vector3{x: 1, y: 2, z: 3};
    // let p2 = Vector3{x: 1, y: 1, z: 1};

    // let pair1 = Pair{p1: p1.clone(), p2: p2.clone()};
    // let pair2 = Pair{p1: p2.clone(), p2: p1.clone()};

    // println!("{:?}: {:x}", &pair1, hash(&pair1));
    // println!("{:?}: {:x}", &pair2, hash(&pair2));
    // return;

    let content = fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<_> = content.lines().collect();

    let mut total = 0;
    let mut circuits: Vec<Circuit> = vec![];
    for line in lines {
        //println!("Line: {}", line);
        let junction = Vector3::from_string(line);
        let mut circuit = Circuit { junctions: vec![] };
        circuit.junctions.push(junction);
        circuits.push(circuit);
    }

    for iteration in 0..10 {
        let shortest_connection = shortest_distance(&circuits);
        
        let circuit_idx = circuit_index_by_junction(&circuits, &shortest_connection.0);
        let circuit2_idx = circuit_index_by_junction(&circuits, &shortest_connection.1);

        let circuit2 = circuits[circuit2_idx].clone();
        let circuit = &mut circuits[circuit_idx];
        circuit.extend(&circuit2);

        circuits.remove(circuit2_idx);
    }

    println!("Final circuits: {:?}", circuits);
    println!("Total: {:?}", circuits.len());

    for circuit in circuits {
        println!("Test: {:?}", circuit.junctions.len());
    }

    println!();
    println!("Total: {}", total);
}

fn shortest_distance(circuits: &Vec<Circuit>) -> (Vector3, Vector3) {
    let mut junctions: Vec<&Vector3> = vec![];
    let mut distance_pairs: HashMap<(Vector3, Vector3), u64> = HashMap::new();
    for circuit in circuits {
        junctions.extend(&circuit.junctions);
    }

    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let first = junctions[i].clone();
            let second = junctions[j].clone();

            let distance = first.distance(&second);
            distance_pairs.insert((first, second), distance as u64);
            //println!("Comparing {:?} to {:?} => {}", junctions[i], junctions[j], distance);
        }
    };

    //println!("Distance pairs: {:#?}", distance_pairs);

    let mut sorted_pairs = distance_pairs.iter().collect::<Vec<_>>();
    sorted_pairs.sort_by_cached_key(|element| {
        element.1
    });

    //println!("Sorted pairs: {:#?}", sorted_pairs);
    sorted_pairs.first().unwrap().0.clone()
}

fn circuit_index_by_junction<'a>(circuits: &Vec<Circuit>, junction: &'a Vector3) -> usize {
    for (index, circuit) in circuits.iter().enumerate() {
        if circuit.junctions.contains(junction) {
            return index;
        }
    }
    panic!("No circuit found for junction {:?}", junction);
}
