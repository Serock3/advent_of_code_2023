#![allow(unused)]

use core::num;

use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("input/day5.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

#[derive(Debug, Clone)]
struct Interval {
    pub start: i64,
    pub len: i64,
}

#[derive(Debug, Clone)]
struct Map {
    pub dest_start: i64,
    pub source_start: i64,
    pub len: i64,
}

#[derive(Debug, Clone)]
struct MapCollection {
    pub layer_name: String,
    pub maps: Vec<Map>,
}

fn solve(input: &str) -> i64 {
    let intervals = parse_intervals(input);

    let map_category = parse_map_categories(input);

    intervals
        .into_iter()
        .enumerate()
        .map(|(i, interval)| {
            println!("On interval nr {i}");
            (interval.start..(interval.start + interval.len))
                .into_par_iter()
                .map(|mut seed| {
                    for maps in &map_category {
                        for map in &maps.maps {
                            let offset = seed - map.source_start;
                            if offset >= 0 && offset < map.len {
                                seed = map.dest_start + offset;
                                break;
                            }
                        }
                    }
                    seed
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn parse_map_categories(input: &str) -> Vec<MapCollection> {
    let layer: Vec<MapCollection> = input
        .split("\n\n")
        .skip(1)
        .map(|segment| {
            let mut lines = segment.lines();
            MapCollection {
                layer_name: lines.next().unwrap().to_string(),
                maps: lines
                    .map(|line| {
                        let (dest_start, source_start, len) = line
                            .trim()
                            .split_ascii_whitespace()
                            .map(|num_str| num_str.parse::<i64>().unwrap())
                            .collect_tuple()
                            .unwrap();
                        Map {
                            dest_start,
                            source_start,
                            len,
                        }
                    })
                    .collect_vec(),
            }
        })
        .collect_vec();
    layer
}

fn parse_intervals(input: &str) -> Vec<Interval> {
    let mut intervals = input
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
    intervals
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
