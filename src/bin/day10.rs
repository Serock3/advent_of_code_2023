#![allow(unused)]

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

struct Pipe {
    r: usize,
    c: usize,
    shape: char,
}

// enum PipeShape{
//     Vertical,
//     Horizontal,
//     NorthEast
// }

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
    let border = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    assert_eq!(char_matrix[s_row][s_col], 'S');
    todo!()
}

fn follow_pipe(r: usize, c: usize, char_matrix: &[Vec<char>]) -> Option<(usize, usize, usize)> {
    todo!()
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
