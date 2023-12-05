#![allow(unused)]

use core::num;

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day5.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

#[derive(Debug)]
struct Interval {
    pub start: i64,
    pub len: i64,
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
        .chunks(2)
        .into_iter()
        .map(|mut nums| Interval {
            start: nums.next().unwrap(),
            len: nums.next().unwrap(),
        })
        .collect_vec();
    dbg!(&seeds);

    let maps = input.split("\n\n").skip(1);
    for seed in &mut seeds {
        for segment in maps.clone() {
            let mut lines = segment.lines();
            println!("{}", lines.next().unwrap());
            for line in lines {
                // let (dest_start, source_start, len): (i64, i64, i64) = line
                //     .trim()
                //     .split_ascii_whitespace()
                //     .map(|num_str| num_str.parse::<i64>().unwrap())
                //     .collect_tuple()
                //     .unwrap();
                // let offset = *seed - source_start;
                // if offset >= 0 && offset < len {
                //     *seed = dest_start + offset;
                //     break;
                // }
            }
        }
    }
    dbg!(&seeds);
    // seeds.into_iter().min().unwrap()
    todo!()
}

fn rec_solve<'a>(interval: Interval, mut maps: &'a str) -> Option<i64> {
    let Some((segment, rest)) = maps.split_once("\n\n") else {
        return None;
    };

    let mut lines = segment.lines();
    println!("{}", lines.next().unwrap());

    lines
        .filter_map(|line| {
            let (dest_start, source_start, len): (i64, i64, i64) = line
                .trim()
                .split_ascii_whitespace()
                .map(|num_str| num_str.parse::<i64>().unwrap())
                .collect_tuple()
                .unwrap();

            let offset_left = interval.start - source_start;
            if offset_left < 0 {
                // First point left of source
                if interval.start + interval.len <= source_start {
                    // no overlap, all to the left
                    rec_solve(interval, rest)
                } else if interval.start + interval.len < source_start + len {
                    // Partial overlap, split to the left
                    let mapped_interval_left = Interval {
                        start: interval.start,
                        len: offset_left,
                    };
                    let mapped_interval_right = Interval {
                        start: interval.start + offset_left,
                        len: interval.len + offset_left,
                    };

                    rec_solve(mapped_interval_left, maps)
                        .min(rec_solve(mapped_interval_right, maps))
                } else {
                    // interval encloses source interval
                    todo!()
                }
            } else if interval.start <= source_start + len {
                // First point inside source interval
                if interval.start + interval.len < source_start + len {
                    // source interval encloses interval
                    todo!()
                } else {
                    //  partial overlap to the right
                    todo!()
                }
            } else {
                // No overlap, all to the right
                rec_solve(interval, rest)
            }

            // if interval.start + interval.len <= source_start || interval.start > source_start + len
            // {
            //     // no overlap
            //     rec_solve(interval, rest)
            // } else if interval.start >= source_start
            //     && interval.start + interval.len <= source_start + len
            // {
            //     // source contains interval
            //     let mapped_interval = Interval {
            //         start: interval.start - source_start,
            //         len: interval.len,
            //     };
            //     rec_solve(&mapped_interval, maps)
            // } else if interval.start < source_start
            //     && interval.start + interval.len > source_start + len
            // {
            //     // interval contains source
            // } else {
            //     None
            // }
            // interval.start < source_start
            //     && interval.start + interval.len > source_start + len

            // let offset = *interval.start - source_start;
            // if offset >= 0 && offset < interval.len {
            //     *seed = dest_start + offset;
            //     break;
            // }
        })
        .min()
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
    assert_eq!(solve(input), 46)
}
