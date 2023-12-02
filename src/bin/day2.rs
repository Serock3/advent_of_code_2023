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
        let (_start, rest) = line.split_once(": ").unwrap();
        // let id: u32 = start.split_once(' ').unwrap().1.parse().unwrap();
        // println!("{}", id);
        for set in rest.split("; ") {
            // dbg!(set);
            let items = set.split(", ").collect_vec();
            // dbg!(&items);

            for x in items {
                let (num, color) = x.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                // println!("{} {}", num, color);
                let max_num = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!(""),
                };
                if num > max_num {
                    // println!("Fail with id {id}");
                    continue 'game;
                }
            }
            // if let Some(r_i) = set.find(" red") {
            //     let r_n = set.chars().collect_vec()[r_i - 1].parse();
            // }
        }
        sum_impossible += id;
    }
    sum_impossible
}

fn solve_2(input: &str) -> usize {
    let mut sum_power = 0;
    'game: for (id, line) in input.lines().enumerate().map(|(id, line)| (id + 1, line)) {
        let (_start, rest) = line.split_once(": ").unwrap();
        // let id: u32 = start.split_once(' ').unwrap().1.parse().unwrap();
        // println!("{}", id);
        let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);
        for set in rest.split("; ") {
            // dbg!(set);
            let items = set.split(", ").collect_vec();
            // dbg!(&items);

            for x in items {
                let (num, color) = x.split_once(' ').unwrap();
                let num: u32 = num.parse().unwrap();
                // println!("{} {}", num, color);
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
            // if let Some(r_i) = set.find(" red") {
            //     let r_n = set.chars().collect_vec()[r_i - 1].parse();
            // }
        }
        let power = max_red * max_green * max_blue;
        // dbg!(power);
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
