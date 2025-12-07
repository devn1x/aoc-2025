use std::{collections::HashMap, fmt::Display, fs};

//const INPUT_PATH: &str = "input/07/example.txt";
const INPUT_PATH: &str = "input/07/input.txt";

#[derive(Debug)]
struct Manifold {
    width: usize,
    layers: usize,
    beams: Vec<Beam>,
    splitters: Vec<Splitter>
}

impl Manifold {
    fn run(self: &mut Self) -> u64 {
        let mut total = 0;

        let beams: Vec<_> = self.beams.iter_mut().filter(|element| {
            !element.ended
        }).collect();

        let mut new_beams: HashMap<(usize, usize), Beam> = HashMap::new();
        for beam in beams {
            let splitter_below = self.splitters.iter().filter(|element| {
                (element.line == beam.line+1 || element.line == beam.line-1) && element.index == beam.index
            }).collect::<Vec<_>>().len() > 0;

            if splitter_below {
                // TODO: Remove duplicates
                beam.ended = true;
                let beam_left = Beam {
                    line: beam.line + 1,
                    index: beam.index - 1,
                    ended: false
                };
                let beam_right = Beam {
                    line: beam.line + 1,
                    index: beam.index + 1,
                    ended: false
                };
                let left_insert = new_beams.insert((beam_left.line, beam_left.index), beam_left).is_some();
                let right_insert = new_beams.insert((beam_right.line, beam_right.index), beam_right).is_some();
                if left_insert || right_insert {
                    //
                }
                total += 1;
            } else {
                beam.ended = true;
                let new_beam = Beam {
                    line: beam.line + 1,
                    index: beam.index,
                    ended: false
                };
                new_beams.insert((new_beam.line, new_beam.index), new_beam);
            }
        }

        self.beams.extend(new_beams.into_values());

        total
    }
}

impl Display for Manifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line_index in 0..self.layers {
            for index in 0..self.width {
                let has_beam = self
                    .beams
                    .iter()
                    .filter(|element| element.index == index && element.line == line_index)
                    .count()
                    > 0;
                let has_splitter = self
                    .splitters
                    .iter()
                    .filter(|element| element.index == index && element.line == line_index)
                    .count()
                    > 0;
                if has_beam {
                    let _ = write!(f, "|");
                } else if has_splitter {
                    let _ = write!(f, "^");
                } else {
                    let _ = write!(f, ".");
                }
            }
            let _ = write!(f, "\n");
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Beam {
    line: usize,
    index: usize,
    ended: bool,
}

#[derive(Debug)]
struct Splitter {
    line: usize,
    index: usize,
}

pub fn main() {
    let content = fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<_> = content.lines().collect();

    // Parsing
    let mut total = 0;
    let mut manifold = Manifold {
        beams: vec![],
        splitters: vec![],
        width: lines[0].len(),
        layers: lines.len()
    };
    for line_index in 0..content.lines().collect::<Vec<_>>().len() {
        let line = lines[line_index];
        println!("Line: {}", line);
        for char in line.char_indices() {
            match char.1 {
                '.' => {
                    continue;
                }
                'S' => {
                    let beam = Beam {
                        index: char.0,
                        line: line_index + 1,
                        ended: false,
                    };
                    manifold.beams.push(beam);
                }
                '^' => {
                    let splitter = Splitter {
                        index: char.0,
                        line: line_index,
                    };
                    manifold.splitters.push(splitter);
                }
                _ => {
                    panic!("This shouldn't happen.");
                }
            }
        }
    }

    for iteration in 0..manifold.layers {
        total += manifold.run();
        println!();
        println!("----- Iteration {} of {}:\n{}", iteration, manifold.layers - 1, "");
    }

    println!();
    println!("Total: {}", total);
}
