#![allow(unused)]

use std::ops::Div;

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;
impl From<&Direction> for (i32, i32) {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

fn solve(input: &str) -> usize {
    let s_pos_raw = input.find('S').unwrap();
    let char_matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let (mut s_row, mut s_col) = (0, 0);
    'outer: for (r, row) in char_matrix.iter().enumerate() {
        for (c, character) in row.iter().enumerate() {
            if *character == 'S' {
                s_row = r;
                s_col = c;
                break 'outer;
            }
        }
    }
    assert_eq!(char_matrix[s_row][s_col], 'S');

    let start_dirs = [North, East, South, West];
    let mut longest = 0;
    for dir in start_dirs {
        // dbg!(dir.clone());
        let res = follow_pipe(
            (s_row.try_into().unwrap(), s_col.try_into().unwrap()),
            dir,
            &char_matrix,
        );
        // dbg!(res.clone());
        if let Some((len, exit_dir)) = res {
            longest = len.max(longest);
        }
    }

    longest.div(2)
}

fn follow_pipe(
    s_pos: (i32, i32),
    mut dir: Direction,
    char_matrix: &[Vec<char>],
) -> Option<(usize, Direction)> {
    let mut pos = add_pos(s_pos, &dir);

    let mut steps = 1;

    loop {
        let pipe = get_pipe(pos, char_matrix)?;
        if pipe == 'S' {
            println!("Found loop");
            return Some((steps, dir));
        }
        dir = match (&dir, pipe) {
            // North
            (North, '|') => North,
            (North, 'F') => East,
            (North, '7') => West,
            // East
            (East, '-') => East,
            (East, '7') => South,
            (East, 'J') => North,
            // South
            (South, '|') => South,
            (South, 'J') => West,
            (South, 'L') => East,
            // West
            (West, '-') => West,
            (West, 'L') => North,
            (West, 'F') => South,

            (dir, sym) => {
                // dbg!((dir, sym));
                return None;
            }
        };
        pos = add_pos(pos, &dir);
        steps += 1;
    }
}

fn add_pos(s_pos: (i32, i32), dir: &Direction) -> (i32, i32) {
    let (r, c) = <(i32, i32)>::from(dir);
    (s_pos.0 + r, s_pos.1 + c)
}

fn get_pipe(pos: (i32, i32), char_matrix: &[Vec<char>]) -> Option<char> {
    let row = usize::try_from(pos.0).ok()?;
    let col = usize::try_from(pos.1).ok()?;

    char_matrix
        .get(row)
        .and_then(|chars| chars.get(col))
        .cloned()
        .filter(|char| *char != '.')
}

#[test]
fn test_example() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    assert_eq!(solve(input), 8)
}

#[test]
fn test_example2() {
    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    assert_eq!(solve(input), 8)
}

#[test]
fn test_example3() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";
    assert_eq!(solve(input), 4)
}
