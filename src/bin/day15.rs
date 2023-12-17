#![allow(unused)]
use advent_of_code::get_input;
use itertools::Itertools;

fn main() {
    println!("Answer: {}", solve(&get_input()));
}

fn solve(input: &str) -> usize {
    input.split(',').map(get_hash).sum()
}

fn get_hash(word: &str) -> usize {
    word.chars()
        .fold(0u32, |acc, c| (acc + c as u32) * 17 % 256) as usize
}

#[test]
fn test_example() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(solve(input), 1320)
}
