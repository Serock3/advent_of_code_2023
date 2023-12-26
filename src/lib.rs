use ndarray::prelude::*;
use std::{ops::Add, panic::Location};

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
use Direction::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos<T>(pub T, pub T);

impl<T: Add<Output = T>> Add for Pos<T> {
    type Output = Pos<T>;

    fn add(self, rhs: Pos<T>) -> Pos<T> {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Pos<isize>> for Pos<usize> {
    type Output = Pos<usize>;

    fn add(self, rhs: Pos<isize>) -> Pos<usize> {
        Pos(
            self.0.saturating_add_signed(rhs.0),
            self.1.saturating_add_signed(rhs.1),
        )
    }
}

impl From<&Direction> for Pos<isize> {
    fn from(value: &Direction) -> Self {
        match value {
            North => Pos(-1, 0),
            East => Pos(0, 1),
            South => Pos(1, 0),
            West => Pos(0, -1),
        }
    }
}
