#![allow(unused)]

use itertools::Itertools;
use num::integer::lcm;
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

    loop_len.into_iter().reduce(lcm).unwrap()
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
