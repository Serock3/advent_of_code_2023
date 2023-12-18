#![allow(unused)]
use std::collections::HashSet;

use advent_of_code::get_input;
use advent_of_code::Direction;
use itertools::Itertools;
use ndarray::s;
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
    let visited = HashSet::new();
    let mut queue = vec![Beam {
        start: Pos(0, 0),
        dir: Direction::East,
    }];

    while let Some(beam) = queue.pop() {
        match beam.dir {
            Direction::North => {
                if let Some((p, c)) = matrix
                    .slice(s![beam.start.0, ..beam.start.1])
                    .iter()
                    .find_position(|&&c| c != '.')
                {
                    let next = Beam {
                        dir: todo!(),
                        start: Pos(beam.start.0, p),
                    };
                    queue.push(next);
                } else {
                    break;
                }
            }
            Direction::East => todo!(),
            Direction::South => todo!(),
            Direction::West => todo!(),
        }

        if visited.insert(beam) {
            todo!()
        }
    }
    todo!()
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
