#![allow(unused)]

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day4.txt").unwrap();
    // println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input));
}

fn solve_2(input: &str) -> usize {
    let mut lines_with_count = input.lines().map(|line| (1, line)).collect_vec();

    for line_num in 0..lines_with_count.len() {
        let (count, line) = lines_with_count[line_num];
        let content = line.split_once(": ").unwrap().1;
        let (winning_nums_str, your_nums) = content.split_once(" | ").unwrap();
        let winning_nums = winning_nums_str
            .split_ascii_whitespace()
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect_vec();

        let num_winning: u32 = your_nums
            .split_ascii_whitespace()
            .map(|num_str| {
                let parsed = num_str.parse::<u32>().unwrap();
                winning_nums.contains(&parsed) as u32
            })
            .sum();

        for i in 0..num_winning {
            if let Some(x) = lines_with_count.get_mut(line_num + i as usize + 1) {
                x.0 += count;
            }
        }
    }

    lines_with_count
        .iter()
        .map(|(count, _line)| *count as usize)
        .sum()
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let content = line.split_once(": ").unwrap().1;
            let (winning_nums_str, your_nums) = content.split_once(" | ").unwrap();
            let winning_nums = winning_nums_str
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect_vec();

            let num_winning: u32 = your_nums
                .split_ascii_whitespace()
                .map(|num_str| {
                    let parsed = num_str.parse::<u32>().unwrap();
                    winning_nums.contains(&parsed) as u32
                })
                .sum();
            if num_winning != 0 {
                2usize.pow(num_winning - 1)
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn test_example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(solve(input), 13)
}

#[test]
fn test_example_2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(solve_2(input), 30)
}
