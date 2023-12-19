#![allow(unused)]
use std::collections::HashSet;
use std::f32::consts::E;

use advent_of_code::get_input;
use advent_of_code::Direction;
use advent_of_code::Direction::*;
use itertools::Itertools;
use ndarray::s;
use ndarray::Array2;
use ndarray::AssignElem;
use ndarray::Axis;

fn main() {
    println!("Answer: {}", solve(&get_input()));
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos<T>(T, T);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Beam {
    start: Pos<usize>,
    dir: Direction,
}

fn solve(input: &str) -> usize {
    let matrix = advent_of_code::parse_char_matrix(input);
    let mut heated = Array2::from_elem(matrix.raw_dim(), false);

    let mut visited = HashSet::new();

    let mut queue = vec![Beam {
        start: Pos(0, 0),
        dir: Direction::East,
    }];

    while let Some(inbound_beam) = queue.pop() {
        match inbound_beam.dir {
            North => {
                let mut slice = s![inbound_beam.start.0, ..inbound_beam.start.1];

                if let Some((p, c)) = matrix.slice(slice).iter().find_position(|&&c| c != '.') {
                    slice = s![inbound_beam.start.0, p..inbound_beam.start.1];
                    let next_start = Pos(inbound_beam.start.0, p);

                    match c {
                        '/' => queue.push(Beam {
                            dir: East,
                            start: next_start,
                        }),
                        '\\' => queue.push(Beam {
                            dir: West,
                            start: next_start,
                        }),
                        '|' => queue.push(Beam {
                            dir: North,
                            start: next_start,
                        }),
                        '-' => {
                            queue.push(Beam {
                                dir: East,
                                start: next_start.clone(),
                            });
                            queue.push(Beam {
                                dir: West,
                                start: next_start,
                            });
                        }
                        _ => panic!(),
                    }
                } else {
                    heated.slice_mut(slice).fill(true);
                }
            }
            East => todo!(),
            South => {
                let mut slice = s![inbound_beam.start.0, inbound_beam.start.1..];

                let maybe_new_beams = if let Some((p, c)) =
                    matrix.slice(slice).iter().find_position(|&&c| c != '.')
                {
                    slice = s![inbound_beam.start.0, inbound_beam.start.1..p];
                    let mirror_pos = Pos(inbound_beam.start.0, p);

                    let next_dirs: &[Direction] = get_reflection_dir(inbound_beam.dir, *c);
                    Some(next_dirs.iter().cloned().map(move |dir| Beam {
                        start: mirror_pos.clone(),
                        dir,
                    }))
                } else {
                    None
                };
                if let Some(new_beams) = maybe_new_beams {
                    queue.extend(new_beams);
                }
                heated.slice_mut(slice).fill(true);
            }
            Direction::West => todo!(),
        }

        if visited.insert(inbound_beam) {
            todo!()
        }
    }
    todo!()
}

fn get_reflection_dir(inbond_dir: Direction, c: char) -> &'static [Direction] {
    match c {
        '/' => match inbond_dir {
            North => &[East],
            East => &[North],
            South => &[West],
            West => &[South],
        },
        '\\' => match inbond_dir {
            North => &[West],
            East => &[South],
            South => &[East],
            West => &[North],
        },
        '|' => match inbond_dir {
            North => &[North],
            East => &[North, South],
            South => &[South],
            West => &[North, South],
        },
        '-' => match inbond_dir {
            North => &[West, East],
            East => &[East],
            South => &[West, East],
            West => &[West],
        },
        _ => panic!(),
    }
}

#[test]
fn test_example() {
    let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
    assert_eq!(solve(input), 46)
}
