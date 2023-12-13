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
    let lines = input.lines().collect_vec();

    let mut horizontal_reflection: Vec<(usize, Rev<Iter<'_, &str>>)> = vec![];
    for (i_line, line) in lines.iter().enumerate() {
        for ref_i in 0..horizontal_reflection.len() {
            let (start_line, it) = &mut horizontal_reflection[ref_i];
            let remove = match it.next() {
                Some(reflected_line) => reflected_line != line,
                None => return *start_line,
            };
            if remove {
                horizontal_reflection.remove(ref_i);
            }
        }

        if let Some(next_line) = lines.get(i_line + 1) {
            if line == next_line {
                let rev = lines[0..i_line + 1].iter().rev();
                horizontal_reflection.push((i_line + 1, rev))
            }
        }
    }

    assert_eq!(horizontal_reflection.len(), 1);
    horizontal_reflection[0].0 * 100
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
