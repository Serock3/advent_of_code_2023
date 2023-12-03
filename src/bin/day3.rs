#![allow(unused)]

use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    // println!("Answer 2: {}", solve_2(&input));
}

//  521242 too low

// line 6
// 1 at 35,6 is not a part
// 4 at 29,6 is not a part

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
        println!("line {y}");
        for (start_x, c) in line.char_indices().filter(|(start_x, c)| !c.is_numeric()) {
            // dbg!(num);
            let start_x = line.find(num).unwrap();
            let left_right = [(start_x.saturating_sub(1), y), (start_x + num.len(), y)];

            let line = (start_x.saturating_sub(1)..start_x + num.len() + 1);
            // dbg!(line.clone().collect_vec());
            let above = line.clone().map(|x| (x, y.saturating_sub(1)));
            let below = line.clone().map(|x| (x, y + 1));

            let mut surroundings = above.chain(left_right).chain(below);

            if !surroundings.clone().any(|pos| symbols.contains_key(&pos)) {
                dbg!(surroundings.collect_vec());
                println!("{num} at {start_x},{y} is not a part");
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
