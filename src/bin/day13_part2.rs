#![allow(unused)]
use std::{iter::Rev, slice::Iter};

use advent_of_code::{get_input, transpose};
use itertools::Itertools;

fn main() {
    println!("Answer: {}", solve(&get_input()));
}

fn solve(input: &str) -> usize {
    input.split("\n\n").map(solve_pattern).sum()
}

fn parse_matrices(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let rows = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let cols = transpose(rows.clone());
    (rows, cols)
}

fn solve_pattern(input: &str) -> usize {
    let (rows, cols) = parse_matrices(input);

    for i in 1.. {
        if i < rows.len() && reflection_at(i, &rows) {
            return i * 100;
        }
        if i < cols.len() && reflection_at(i, &cols) {
            return i;
        }
    }
    panic!()
}

fn reflection_at(row: usize, matrix: &[Vec<char>]) -> bool {
    let (upper, lower) = matrix.split_at(row);

    let mut found_smudge = false;
    for (reflected_line, line) in upper.iter().rev().zip(lower) {
        match reflected_line
            .iter()
            .zip(line)
            .filter(|(c1, c2)| c1 != c2)
            .count()
        {
            0 => {}
            1 => {
                if found_smudge {
                    return false;
                }
                found_smudge = true;
            }
            _ => return false,
        }
    }
    found_smudge
}

#[test]
fn test_reflector() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
    let (rows, cols) = parse_matrices(input);
    assert!(reflection_at(3, &rows));
    assert!(!reflection_at(4, &rows));
    assert!(!reflection_at(5, &cols));
}

#[test]
fn test_reflector2() {
    let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    let (rows, cols) = parse_matrices(input);
    assert!(reflection_at(1, &rows));
    assert!(!reflection_at(4, &rows));
}

#[test]
fn test_example() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(solve(input), 400)
}
