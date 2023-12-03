#![allow(unused)]

use std::fmt::format;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input/day1.txt");
    // let input = std::fs::read_to_string("input/day1.txt").unwrap();
    // println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(input));
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            format!(
                "{}{}",
                line.chars().find(|c| c.is_ascii_digit()).unwrap(),
                line.chars().rev().find(|c| c.is_ascii_digit()).unwrap(),
            )
            .parse::<usize>()
            .unwrap()
        })
        .sum()
}

fn solve_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let digits = (1..10).map(|i| char::from_digit(i as u32, 10).unwrap());
            let digit_pos = (1..10)
                .map(|i| {
                    (
                        i as usize,
                        line.find(char::from_digit(i as u32, 10).unwrap()),
                    )
                })
                .filter(|(i, n)| n.is_some())
                .map(|(i, n)| (i, n.unwrap()));

            // let asd = digit_pos.clone().collect_vec();
            // println!("{asd:?}");

            let spelled_digits = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
            ];
            let spelled_digits_pos = spelled_digits
                .into_iter()
                .map(|s| line.find(s))
                .enumerate()
                .filter(|(i, n)| n.is_some())
                .map(|(i, n)| (i, n.unwrap()))
                .map(|(i, n)| (i + 1, n));

            let (first, n) = digit_pos
                .chain(spelled_digits_pos)
                .min_by_key(|(i, n)| *n)
                .unwrap();

            // print!("{first}");

            let mut rev_line = line.chars().rev().collect::<String>();

            let digit_pos = (1..10).map(|i| {
                (
                    i as usize,
                    rev_line.find(|c: char| c.to_digit(10) == Some(i)),
                )
            });
            let spelled_digits = spelled_digits
                .map(|s| s.chars().rev().collect::<String>())
                .into_iter()
                .map(|s| rev_line.find(&s))
                .enumerate()
                .map(|(i, n)| (i + 1, n));

            let (last, n) = digit_pos
                .chain(spelled_digits)
                .filter(|(i, n)| n.is_some())
                .map(|(i, n)| (i, n.unwrap()))
                .min_by_key(|(i, n)| *n)
                .unwrap();

            // println!("{last}");

            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum()
}

#[test]
fn test_example() {
    let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    assert_eq!(solve(input), 142);
}

#[test]
fn test_example_2() {
    let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";
    assert_eq!(solve_2(input), 281)
}
#[test]
fn asd() {
    let input = file!();
    println!("{input:?}")
}
