#![allow(unused)]

use core::num;

use itertools::Itertools;

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
    dbg!(&intervals);

    let maps: Vec<MapCollection> = input
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

    let maps = input.split("\n\n").skip(1);
    for seed in &mut intervals {
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
    dbg!(&intervals);
    // seeds.into_iter().min().unwrap()
    todo!()
}

fn rec_solve(interval: Interval, mut layers_of_maps: &[MapCollection]) -> i64 {
    let Some((map_collection, rest)) = layers_of_maps.split_first() else {
        // No more mapping to do. interval.start is always the lowest point
        return interval.start;
    };

    println!("{}", map_collection.layer_name);

    // TODO: If the interval is split, map the part that is overlapping and CONTINUE to see if the unmapped parts get mapped by some other map
    for map in &map_collection.maps {
        let offset_left = interval.start - map.source_start;
        if offset_left < 0 {
            // First point left of source
            if interval.start + interval.len <= map.source_start {
                // no overlap, all to the left
                continue;
            } else if interval.start + interval.len < map.source_start + map.len {
                // Partial overlap, split to the left
                let mapped_interval_left = Interval {
                    start: interval.start,
                    len: offset_left,
                };
                let mapped_interval_right = Interval {
                    start: interval.start + offset_left,
                    len: interval.len + offset_left,
                };

                return rec_solve(mapped_interval_left, layers_of_maps)
                    .min(rec_solve(mapped_interval_right, layers_of_maps));
            } else {
                // interval encloses source interval
                todo!()
            }
        } else if interval.start <= map.source_start + map.len {
            // First point inside source interval
            if interval.start + interval.len < map.source_start + map.len {
                // source interval encloses interval
                todo!()
            } else {
                //  partial overlap to the right
                todo!()
            }
        } else {
            // No overlap, all to the right
            continue;
        }
    }
    rec_solve(interval, rest)
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
