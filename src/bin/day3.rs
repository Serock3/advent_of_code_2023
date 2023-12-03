#![allow(unused)]

use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    // println!("Answer 2: {}", solve_2(&input));
}

//  521242 too low

fn solve(input: &str) -> usize {
    let mut symbols = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.match_indices(|c: char| !c.is_numeric() && c != '.')
        // .split(|c: char| !c.is_numeric())
        {
            // println!("{:?}", (x, y, symbol));
            if let Some(double) = symbols.insert((x, y), symbol) {
                panic!("{double}")
            }
        }
    }
    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        for num in line
            .split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
        {
            // dbg!(num);
            let start_x = line.find(num).unwrap();
            let left_right = [(start_x.saturating_sub(1), y), (start_x + num.len(), y)];

            let line = (start_x.saturating_sub(1)..start_x + num.len() + 1);
            // dbg!(line.clone().collect_vec());
            let above = line.clone().map(|x| (x, y.saturating_sub(1)));
            let below = line.clone().map(|x| (x, y + 1));

            let mut surroundings = above.chain(left_right).chain(below);

            if surroundings.any(|pos| symbols.contains_key(&pos)) {
                println!("{num} at {start_x},{y} is a part");
                sum += num.parse::<usize>().unwrap();
            }
        }
    }

    sum
}

#[test]
fn test_example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(solve(input), 4361)
}
