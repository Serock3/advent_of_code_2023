#![allow(unused)]

use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/day8.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let rl = lines.next().unwrap().trim().chars().cycle();
    let mut map = HashMap::new();
    let mut ends_with_a = vec![];
    for line in lines.skip(1) {
        let name = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];
        if name.ends_with('A') {
            ends_with_a.push(name);
        }
        assert_eq!(map.insert(name, (left, right)), None);
    }
    let mut counter = 0;
    let mut current_room = "AAA";
    let loop_len = ends_with_a
        .iter()
        .cloned()
        .map(|mut current_room| {
            let mut counter = 0;
            for dir in rl.clone() {
                counter += 1;
                match dir {
                    'L' => current_room = map[current_room].0,
                    'R' => current_room = map[current_room].1,
                    _ => panic!(),
                }

                if current_room.ends_with('Z') {
                    return counter;
                }
            }
            panic!()
        })
        .collect_vec();

    lcm(&loop_len)
}

// Courtesy of https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[test]
fn test_example() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(solve(input), 6)
}
