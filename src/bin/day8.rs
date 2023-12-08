#![allow(unused)]

use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/day8.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let rl = lines.next().unwrap().trim().chars().cycle();
    let mut map = HashMap::new();
    for line in lines.skip(1) {
        let name = &line[..3];
        let left = &line[7..10];
        let right = &line[12..15];
        assert_eq!(map.insert(name, (left, right)), None);
    }
    let mut counter = 0;
    let mut current_room = "AAA";
    for dir in rl {
        counter += 1;
        match dir {
            'L' => current_room = map[current_room].0,
            'R' => current_room = map[current_room].1,
            _ => panic!(),
        }

        if current_room == "ZZZ" {
            return counter;
        }
    }
    todo!()
}

#[test]
fn test_example() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(solve(input), 2)
}

#[test]
fn test_example2() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(solve(input), 6)
}
