#![allow(unused)]
use std::{iter::Rev, slice::Iter};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day13.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

fn solve(input: &str) -> usize {
    input.split("\n\n").map(solve_pattern).sum()
}

fn solve_pattern(input: &str) -> usize {
    let rows = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let cols = transpose(rows.clone());

    let mut col_reflection: Vec<(usize, Rev<Iter<'_, Vec<char>>>)> = vec![];
    let mut row_reflection: Vec<(usize, Rev<Iter<'_, Vec<char>>>)> = vec![];
    let mut row_iter = rows.iter().enumerate().fuse();
    let mut col_iter = cols.iter().enumerate().fuse();
    loop {
        match row_iter.next() {
            Some((i_row, row)) => {
                for ref_i in (0..row_reflection.len()).rev() {
                    let (start_row, it) = &mut row_reflection[ref_i];
                    let remove = match it.next() {
                        Some(reflected_row) => reflected_row != row,
                        None => return *start_row * 100,
                    };
                    if remove {
                        row_reflection.remove(ref_i);
                    }
                }

                if let Some(next_row) = rows.get(i_row + 1) {
                    if row == next_row {
                        let rev = rows[0..i_row + 1].iter().rev();
                        row_reflection.push((i_row + 1, rev))
                    }
                }
            }
            None => {
                if !row_reflection.is_empty() {
                    assert_eq!(row_reflection.len(), 1);
                    return row_reflection[0].0 * 100;
                }
            }
        }
        match col_iter.next() {
            Some((i_col, col)) => {
                for ref_i in (0..col_reflection.len()).rev() {
                    let (start_col, it) = &mut col_reflection[ref_i];
                    let remove = match it.next() {
                        Some(reflected_col) => reflected_col != col,
                        None => return *start_col,
                    };
                    if remove {
                        col_reflection.remove(ref_i);
                    }
                }

                if let Some(next_col) = cols.get(i_col + 1) {
                    if col == next_col {
                        let rev = cols[0..i_col + 1].iter().rev();
                        col_reflection.push((i_col + 1, rev))
                    }
                }
            }
            None => {
                if !col_reflection.is_empty() {
                    assert_eq!(col_reflection.len(), 1);
                    return col_reflection[0].0;
                }
            }
        }
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
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

#[test]
fn test_example_single() {
    let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(solve_pattern(input), 400)
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
    assert_eq!(solve(input), 405)
}
