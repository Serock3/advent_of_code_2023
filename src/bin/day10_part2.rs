#![allow(unused)]

use std::ops::Div;

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

#[derive(Debug, Clone, PartialEq)]
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
    let (_len, dir, area) = start_dirs
        .into_iter()
        .filter_map(|dir| {
            follow_pipe(
                (s_row.try_into().unwrap(), s_col.try_into().unwrap()),
                dir,
                &char_matrix,
            )
        })
        .max_by(|(len_x, _, _), (len_y, _, _)| len_x.cmp(len_y))
        .unwrap();

    area.try_into().unwrap()
}

fn follow_pipe(
    mut pos: (i32, i32),
    mut dir: Direction,
    char_matrix: &[Vec<char>],
) -> Option<(usize, Direction, u32)> {
    let start_dir = dir.clone();
    let mut steps = 0;

    let mut twice_area = 0;
    let mut num_straight = 0;
    let mut num_bends = 0;
    loop {
        // Take step
        steps += 1;

        twice_area += twise_area_change(pos, &dir);

        pos = add_pos(pos, &dir);
        let pipe = get_pipe(pos, char_matrix)?;
        match pipe {
            '|' => num_straight += 1,
            '-' => num_straight += 1,
            'L' => num_bends += 1,
            'J' => num_bends += 1,
            '7' => num_bends += 1,
            'F' => num_bends += 1,
            'S' => {
                if dir == start_dir {
                    num_straight += 1
                } else {
                    num_bends += 1
                }
            }
            _ => {}
        }
        dir = match (dir, pipe) {
            (dir, 'S') => {
                return Some((
                    steps,
                    dir,
                    get_gap_area(
                        (twice_area.abs() / 2).try_into().unwrap(),
                        num_straight,
                        num_bends,
                    ),
                ))
            }
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
    }
}

fn get_gap_area(enclosed_area: u32, num_straight: u32, num_bends: u32) -> u32 {
    let num_extra_bends = num_bends - 4;
    enclosed_area - (num_straight + num_extra_bends) / 2 - 1
}

fn twise_area_change(pos: (i32, i32), dir: &Direction) -> i32 {
    let (dr, dc) = <(i32, i32)>::from(dir);
    let (dx, dy) = (dc, -dr);
    let (x, y) = (pos.1, -pos.0);
    x * dy - y * dx
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
fn test_gap_area() {
    assert_eq!(get_gap_area(10, 6, 10), 3)
}

#[test]
fn test_gap_area2() {
    assert_eq!(get_gap_area(9, 8, 8), 2)
}

#[test]
fn test_example() {
    let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    assert_eq!(solve(input), 4)
}

#[test]
fn test_example2() {
    let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    assert_eq!(solve(input), 8)
}

#[test]
fn test_example3() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";
    assert_eq!(solve(input), 1)
}
