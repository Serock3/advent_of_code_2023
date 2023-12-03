#![allow(unused)]

use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day3.txt").unwrap();
    // println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input));
}

fn solve(input: &str) -> usize {
    let mut symbols = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.match_indices(|c: char| !c.is_numeric() && c != '.') {
            // println!("{:?}", (x, y, symbol));
            if let Some(double) = symbols.insert((x, y), symbol) {
                panic!("{double}")
            }
        }
    }
    let mut sum = 0;
    for (y, line) in input.lines().enumerate() {
        println!("line {y}");
        let mut nums_with_pos: Vec<(usize, String)> = vec![];
        let mut previous_char_was_num = false;
        for (x, c) in line.char_indices() {
            if !c.is_numeric() {
                previous_char_was_num = false;
                continue;
            }
            if !previous_char_was_num {
                nums_with_pos.push((x, String::from(c)));
                previous_char_was_num = true;
            } else {
                nums_with_pos.last_mut().unwrap().1.push(c);
            }
        }
        for (start_x, num) in nums_with_pos {
            let left_right = [(start_x.saturating_sub(1), y), (start_x + num.len(), y)];

            let line = (start_x.saturating_sub(1)..start_x + num.len() + 1);
            let above = line.clone().map(|x| (x, y.saturating_sub(1)));
            let below = line.clone().map(|x| (x, y + 1));

            let mut surroundings = above.chain(left_right).chain(below);

            if surroundings.any(|pos| symbols.contains_key(&pos)) {
                sum += num.parse::<usize>().unwrap();
            } else {
                println!("{num} at {start_x},{y} is not a part");
            }
        }
    }

    sum
}

fn solve_2(input: &str) -> usize {
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, _symbol) in line.match_indices('*') {
            // println!("{:?}", (x, y, symbol));
            if let Some(double) = gears.insert((x, y), vec![]) {
                panic!("{double:?}")
            }
        }
    }

    for (y, line) in input.lines().enumerate() {
        println!("line {y}");
        let mut nums_with_pos: Vec<(usize, String)> = vec![];
        let mut previous_char_was_num = false;
        for (x, c) in line.char_indices() {
            if !c.is_numeric() {
                previous_char_was_num = false;
                continue;
            }
            if !previous_char_was_num {
                nums_with_pos.push((x, String::from(c)));
                previous_char_was_num = true;
            } else {
                nums_with_pos.last_mut().unwrap().1.push(c);
            }
        }

        for (start_x, num) in nums_with_pos {
            let left_right = [(start_x.saturating_sub(1), y), (start_x + num.len(), y)];

            let line = (start_x.saturating_sub(1)..start_x + num.len() + 1);
            let above = line.clone().map(|x| (x, y.saturating_sub(1)));
            let below = line.clone().map(|x| (x, y + 1));

            let mut surroundings = above.chain(left_right).chain(below);

            for pos in surroundings {
                if let Some(gear_nums) = gears.get_mut(&pos) {
                    gear_nums.push(num.parse().unwrap());
                    println!("* at {},{} is has num {num}", pos.0, pos.1);
                    break;
                }
            }
        }
    }

    gears
        .iter()
        .filter_map(|(_pox, nums)| {
            if nums.len() == 2 {
                Some(nums[0] * nums[1])
            } else {
                None
            }
        })
        .sum()
}

#[test]
fn test_example_2() {
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
    assert_eq!(solve_2(input), 467835)
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
