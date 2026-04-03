use std::{collections::HashMap, fs, hash::{DefaultHasher, Hash, Hasher}, usize};

//const INPUT_PATH: &str = "input/08/example.txt";
const INPUT_PATH: &str = "input/08/input.txt";

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64
}

#[derive(Debug, Eq, Clone)]
struct Pair<T: PartialOrd + Hash> {
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
    fn distance(self: &Self, other: &Self) -> i64 {
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

impl<T: PartialOrd + Hash> Hash for Pair<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.p2 < self.p1 {
            self.p2.hash(state);
            self.p1.hash(state);
        } else {
            self.p1.hash(state);
            self.p2.hash(state);
        }
    }
}

impl<T: PartialOrd + Hash> PartialEq for Pair<T> {
    fn eq(&self, other: &Self) -> bool {
        hash(self) == hash(other)
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

    // let pair1 = Pair{p1: &p1.clone(), p2: &p2.clone()};
    // let pair2 = Pair{p1: &p2.clone(), p2: &p1.clone()};

    // println!("{:?}: {:x}", &pair1, hash(&pair1));
    // println!("{:?}: {:x}", &pair2, hash(&pair2));
    // println!("Comparison: {:#}", &pair1 == &pair2);
    // return;

    let content = fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<_> = content.lines().collect();

    let mut circuits: Vec<Vec<Vector3>> = vec![];
    for line in lines {
        //println!("Line: {}", line);
        let junction = Vector3::from_string(line);
        let mut circuit = vec![];
        circuit.push(junction);
        circuits.push(circuit);
    }

    // let mut processed_pairs: Vec<u64> = vec![];
    let mut total = 0;
    let shortest_connections = shortest_distances(&circuits, None);
    for shortest_connection in shortest_connections {
        let circuit_idx = circuit_index_by_junction(&circuits, &shortest_connection.p1);
        let circuit2_idx = circuit_index_by_junction(&circuits, &shortest_connection.p2);

        if circuit_idx != circuit2_idx {
            let circuit2 = circuits[circuit2_idx].clone();
            let circuit = &mut circuits[circuit_idx];
            circuit.extend(circuit2);
            circuits.remove(circuit2_idx);
            // println!("Merged");
        }

        if circuits.len() == 1 {
            // println!("All connected: {}", &shortest_connection.p1.x * &shortest_connection.p2.x);
            total = &shortest_connection.p1.x * &shortest_connection.p2.x;
            break;
        }
    }

    circuits.sort_by_cached_key(|f| f.len());
    circuits.reverse();
    // println!("Final circuits: {:#?}", circuits);
    println!("Total circuits: {:?}", circuits.len());

    println!();
    println!("Total: {}", total);
}

fn shortest_distances(circuits: &Vec<Vec<Vector3>>, limit: Option<usize>) -> Vec<Pair<Vector3>> {
    let mut junctions: Vec<&Vector3> = vec![];
    let mut distance_pairs: HashMap<Pair<Vector3>, u64> = HashMap::new();
    for circuit in circuits {
        junctions.extend(circuit);
    }

    for i in 0..junctions.len() {
        let first = junctions[i].clone();
        for j in (i + 1)..junctions.len() {
            let second = junctions[j].clone();

            let distance = first.distance(&second);
            let pair = Pair { p1: first.clone(), p2: second.clone() };
            distance_pairs.insert(pair, distance as u64);
            //println!("Comparing {:?} to {:?} => {}", junctions[i], junctions[j], distance);
        }
    };

    //println!("Distance pairs: {:#?}", distance_pairs);

    let mut sorted_pairs = distance_pairs.iter().collect::<Vec<_>>();
    sorted_pairs.sort_by_cached_key(|element| {
        element.1
    });

    //println!("Sorted pairs: {:#?}", sorted_pairs);
    let sorted_pairs_iter = sorted_pairs.iter();
    sorted_pairs_iter.take(limit.unwrap_or(usize::MAX)).map(|f| f.0.clone()).collect()
}

fn circuit_index_by_junction<'a>(circuits: &Vec<Vec<Vector3>>, junction: &'a Vector3) -> usize {
    for (index, circuit) in circuits.iter().enumerate() {
        if circuit.contains(junction) {
            return index;
        }
    }
    panic!("No circuit found for junction {:?}", junction);
}
