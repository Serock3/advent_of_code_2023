#![allow(unused)]

use core::num;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day5.txt").unwrap();
    println!("Answer 1: {}", solve(&input));
    // println!("Answer 2: {}", solve_2(&input));
}

fn solve(input: &str) -> i64 {
    let mut seeds = input
        .lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect_vec();
    dbg!(&seeds);

    for seed in &mut seeds {
        for segment in input.split("\n\n").skip(1) {
            let mut lines = segment.lines();
            println!("{}", lines.next().unwrap());
            for line in lines {
                let (dest_start, source_start, len): (i64, i64, i64) = line
                    .trim()
                    .split_ascii_whitespace()
                    .map(|num_str| num_str.parse::<i64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                let offset = *seed - source_start;
                if offset >= 0 && offset < len {
                    *seed = dest_start + offset;
                    break;
                }
            }
        }
    }

    // let maps: Vec<Vec<(i32, i32, i32)>> = input
    //     .split("\n\n")
    //     .skip(1)
    //     .map(|segment| {
    //         segment
    //             .lines()
    //             .skip(1)
    //             .map(|line| {
    //                 line.trim()
    //                     .split_ascii_whitespace()
    //                     .map(|num_str| num_str.parse::<i32>().unwrap())
    //                     .collect_tuple()
    //                     .unwrap()
    //             })
    //             .collect_vec()
    //     })
    //     .collect_vec();

    // dbg!(&maps);

    // for seed in &mut seeds {
    //     for sub_map in &maps {
    //         println!("{seed}");
    //         for (dest_start, source_start, len) in sub_map {
    //             let offset = *seed - *source_start;
    //             if offset >= 0 && offset < *len {
    //                 *seed = dest_start + offset;
    //                 break;
    //             }
    //         }
    //     }
    // }
    dbg!(&seeds);
    seeds.into_iter().min().unwrap()
}

#[test]
fn test_example() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(solve(input), 35)
}
