#![allow(unused)]

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day9.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

fn solve(input: &str) -> i32 {
    input.lines().map(solve_single).sum()
}

fn solve_single(input: &str) -> i32 {
    let nums = input
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect_vec();

    extrapolate(&nums)
}

fn extrapolate(series: &[i32]) -> i32 {
    let diffs = series
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    let next_value = if diffs.iter().all_equal() {
        diffs[0]
    } else {
        extrapolate(&diffs)
    };
    series.first().unwrap() - next_value
}

#[test]
fn test_single() {
    let input = "0 3 6 9 12 15";
    assert_eq!(solve_single(input), -3)
}

#[test]
fn test_single2() {
    let input = "1 3 6 10 15 21";
    assert_eq!(solve_single(input), 0)
}

#[test]
fn test_single3() {
    let input = "10 13 16 21 30 45";
    assert_eq!(solve_single(input), 5)
}

#[test]
fn test_example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(solve(input), 2)
}
