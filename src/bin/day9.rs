#![allow(unused)]

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day9.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

fn solve(input: &str) -> usize {
    input.lines().map(solve_single).sum()
}

fn solve_single(input: &str) -> usize {
    let nums = input
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse::<i32>().unwrap())
        .collect_vec();

    loop {
        let mut diffs = nums.iter().tuple_windows().map(|(a, b)| b - a);
        if diffs.all_equal() {
            println!("All all_equal");
            break;
        }
        dbg!(diffs.collect_vec());
    }

    todo!()
}

#[test]
fn test_single() {
    let input = "0 3 6 9 12 15";
    assert_eq!(solve_single(input), 18)
}

#[test]
fn test_single2() {
    let input = "1 3 6 10 15 21";
    assert_eq!(solve_single(input), 28)
}

#[test]
fn test_single3() {
    let input = "10 13 16 21 30 45";
    assert_eq!(solve_single(input), 68)
}

#[test]
fn test_example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(solve(input), 114)
}
