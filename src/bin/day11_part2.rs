#![allow(unused)]

use itertools::Itertools;
type Pos = (usize, usize);

fn main() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();
    println!("Answer: {}", solve(input, 1000000));
}

fn solve(mut input: String, expansion_rate: usize) -> usize {
    let mut lines = input.lines().collect_vec();
    let matrix = lines
        .iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let empty_rows = matrix
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if !line.contains(&'#') { Some(i) } else { None })
        .collect_vec();

    let empty_columns = transpose(matrix.clone())
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if !line.contains(&'#') { Some(i) } else { None })
        .collect_vec();

    let stars = find_stars(&matrix);
    stars
        .iter()
        .enumerate()
        .combinations(2)
        .map(|x| {
            assert_eq!(x.len(), 2);
            distance(x[0].1, x[1].1, &empty_rows, &empty_columns, expansion_rate)
        })
        .sum()
}

fn find_stars(char_matrix: &[Vec<char>]) -> Vec<Pos> {
    char_matrix
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(c, character)| {
                    if *character == '#' {
                        Some((r, c))
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec()
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

fn distance(
    first: &Pos,
    second: &Pos,
    empty_rows: &[usize],
    empty_cols: &[usize],
    expansion_rate: usize,
) -> usize {
    let lower_row = second.0.min(first.0);
    let higher_row = second.0.max(first.0);
    let lower_col = second.1.min(first.1);
    let higher_col = second.1.max(first.1);
    let dr = higher_row - lower_row;
    let dc = higher_col - lower_col;
    let empty_rows_in_path = empty_rows
        .iter()
        .filter(|r| (lower_row..higher_row).contains(r))
        .count();
    let empty_cols_in_path = empty_cols
        .iter()
        .filter(|c| (lower_col..higher_col).contains(c))
        .count();

    empty_rows_in_path * (expansion_rate - 1) + dr + empty_cols_in_path * (expansion_rate - 1) + dc
}

#[test]
fn test_dist() {
    assert_eq!(distance(&(5, 1), &(9, 4), &[], &[], 2), 7)
}

#[test]
fn test_dist2() {
    assert_eq!(distance(&(5, 1), &(9, 4), &[3, 7], &[2, 5, 8], 2), 9)
}

#[test]
fn test_dist3() {
    assert_eq!(distance(&(2, 0), &(6, 9), &[3, 7], &[2, 5, 8], 2), 17)
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
    assert_eq!(solve(input.into(), 2), 374)
}

#[test]
fn test_example10() {
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
    assert_eq!(solve(input.into(), 10), 1030)
}

#[test]
fn test_example100() {
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
    assert_eq!(solve(input.into(), 100), 8410)
}
