use ndarray::prelude::*;
use std::panic::Location;

#[track_caller]
pub fn get_input() -> String {
    let n = Location::caller()
        .file()
        .strip_prefix("src/bin/day")
        .unwrap()
        .split_once(['_', '.', '-', 'p'])
        .unwrap()
        .0;
    let input_path = format!("input/day{n}.txt");
    std::fs::read_to_string(input_path).unwrap()
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn parse_char_matrix(input: &str) -> Array2<char> {
    let shape = (input.lines().count(), input.lines().next().unwrap().len());

    let chars = input.chars().filter(|c| *c != '\n');
    Array::from_iter(chars).into_shape(shape).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub type Pos = (i32, i32);

impl From<&Direction> for Pos {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}
