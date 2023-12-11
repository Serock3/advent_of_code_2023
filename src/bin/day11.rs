#![allow(unused)]

use itertools::Itertools;
type Pos = (usize, usize);

fn main() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    println!("Answer: {}", solve(input));
}

fn solve(mut input: String) -> usize {
    let mut lines = input.lines().collect_vec();
    let matrix = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let empty_lines = matrix
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if !line.contains(&'#') { Some(i) } else { None })
        .collect_vec();

    let empty_rows = transpose(matrix)
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if !line.contains(&'#') { Some(i) } else { None })
        .collect_vec();

    dbg!(empty_lines);
    dbg!(empty_rows);
    todo!()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn distane(first: Pos, second: Pos, empty_lines: &[usize], empty_rows: &[usize]) -> usize {
    todo!()
}

#[test]
fn test_example() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(solve(input.into()), 374)
}
