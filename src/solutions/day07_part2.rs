use std::{collections::HashMap, fmt::Display, fs};

//const INPUT_PATH: &str = "input/07/example.txt";
const INPUT_PATH: &str = "input/07/input.txt";

#[derive(Debug)]
struct Manifold {
    width: usize,
    layers: usize,
    beams: Vec<Beam>,
    splitters: Vec<Splitter>,
}

impl Manifold {
    fn run(self: &mut Self) {
        let beams: Vec<_> = self
            .beams
            .iter_mut()
            .filter(|element| !element.ended)
            .collect();

        let mut new_beams: HashMap<(usize, usize), Beam> = HashMap::new();
        for beam in beams {
            let splitter_below = self
                .splitters
                .iter()
                .filter(|element| {
                    (element.line == beam.line + 1 || element.line == beam.line - 1)
                        && element.index == beam.index
                })
                .collect::<Vec<_>>()
                .len()
                > 0;

            if splitter_below {
                beam.ended = true;
                let left_beam = new_beams
                    .iter()
                    .filter(|b| b.1.line == beam.line + 1 && b.1.index == beam.index - 1)
                    .collect::<Vec<_>>();
                let left_beam = left_beam.first();
                let right_beam = new_beams
                    .iter()
                    .filter(|b| b.1.line == beam.line + 1 && b.1.index == beam.index + 1)
                    .collect::<Vec<_>>();
                let right_beam = right_beam.first();

                let mut left_multiverses = beam.multiverse_count;
                let mut right_multiverses = beam.multiverse_count;

                if left_beam.is_some() {
                    let left_beam = left_beam.unwrap().1;
                    left_multiverses += left_beam.multiverse_count;
                }
                if right_beam.is_some() {
                    let right_beam = right_beam.unwrap().1;
                    right_multiverses += right_beam.multiverse_count;
                }

                let beam_left = Beam {
                    line: beam.line + 1,
                    index: beam.index - 1,
                    ended: false,
                    multiverse_count: left_multiverses,
                };
                let beam_right = Beam {
                    line: beam.line + 1,
                    index: beam.index + 1,
                    ended: false,
                    multiverse_count: right_multiverses,
                };

                let _left_insert_result =
                    new_beams.insert((beam_left.line, beam_left.index), beam_left);
                let _right_insert_result =
                    new_beams.insert((beam_right.line, beam_right.index), beam_right);

                //total += 1;
            } else {
                beam.ended = true;

                let top_beam = new_beams
                    .iter()
                    .filter(|b| b.1.line == beam.line + 1 && b.1.index == beam.index)
                    .collect::<Vec<_>>();
                let top_beam = top_beam.first();

                let mut top_multiverses = beam.multiverse_count;

                if top_beam.is_some() {
                    let top_beam = top_beam.unwrap().1;
                    top_multiverses += top_beam.multiverse_count;
                }

                let new_beam = Beam {
                    line: beam.line + 1,
                    index: beam.index,
                    ended: false,
                    multiverse_count: top_multiverses,
                };
                new_beams.insert((new_beam.line, new_beam.index), new_beam);
            }
        }

        self.beams.extend(new_beams.into_values());
    }
}

impl Display for Manifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line_index in 0..self.layers {
            for index in 0..self.width {
                let beams: Vec<_> = self
                    .beams
                    .iter()
                    .filter(|element| element.index == index && element.line == line_index)
                    .collect();
                let has_beam = beams.len() > 0;
                let has_splitter = self
                    .splitters
                    .iter()
                    .filter(|element| element.index == index && element.line == line_index)
                    .count()
                    > 0;
                if has_beam {
                    let beam = beams[0];
                    if beam.multiverse_count < 15 {
                        let _ = write!(f, "{:X}", beam.multiverse_count);
                    } else {
                        let _ = write!(f, "|");
                    }
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
    multiverse_count: u64,
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
    let mut manifold = Manifold {
        beams: vec![],
        splitters: vec![],
        width: lines[0].len(),
        layers: lines.len(),
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
                        multiverse_count: 1,
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

    let mut total = 0;
    for iteration in 0..manifold.layers {
        manifold.run();
        println!();
        println!(
            "----- Iteration {} of {}:\n{}",
            iteration,
            manifold.layers - 1,
            ""
        );
        if iteration == manifold.layers - 1 {
            let filtered_beams = manifold.beams.iter().filter(|b| b.line == iteration);
            for beam in filtered_beams {
                total += beam.multiverse_count;
            }
        }
    }

    println!();
    println!("Total: {}", total);
}
