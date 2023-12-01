#![allow(unused)]

fn main() {
    let input = std::fs::read_to_string("input/day??.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    // println!("Answer 2: {}", solve_2(&input));
}

fn solve(input: &str) -> usize {
    todo!()
}

#[test]
fn test_example() {
    let input = "";
    assert_eq!(solve(input), todo!())
}
