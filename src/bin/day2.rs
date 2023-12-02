#![allow(unused)]

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day2.txt").unwrap();
    // println!("Answer 1: {}", solve(&input));
    println!("Answer 2: {}", solve_2(&input));
}

fn solve(input: &str) -> usize {
    let mut sum_impossible = 0;
    'game: for (id, line) in input.lines().enumerate().map(|(id, line)| (id + 1, line)) {
        let (_start, game_input) = line.split_once(": ").unwrap();
        for set in game_input.split("; ") {
            for cubes in set.split(", ") {
                let (num, color) = cubes.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                let max_num = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!(""),
                };
                if num > max_num {
                    continue 'game;
                }
            }
        }
        sum_impossible += id;
    }
    sum_impossible
}

fn solve_2(input: &str) -> usize {
    let mut sum_power = 0;
    for line in input.lines() {
        let (_start, game_input) = line.split_once(": ").unwrap();

        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        for set in game_input.split("; ") {
            for cubes in set.split(", ") {
                let (num, color) = cubes.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                match color {
                    "red" => {
                        if num > max_red {
                            max_red = num;
                        }
                    }
                    "green" => {
                        if num > max_green {
                            max_green = num;
                        }
                    }
                    "blue" => {
                        if num > max_blue {
                            max_blue = num;
                        }
                    }
                    _ => panic!(""),
                };
            }
        }
        let power = max_red * max_green * max_blue;
        sum_power += power as usize;
    }
    sum_power
}

#[test]
fn test_example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(solve(input), 8)
}

#[test]
fn test_example_2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(solve_2(input), 2286)
}
