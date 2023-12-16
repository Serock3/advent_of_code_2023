#![allow(unused)]
use advent_of_code::{get_input, parse_char_matrix};
use itertools::Itertools;
use ndarray::prelude::*;

fn main() {
    println!("Answer: {}", solve(&get_input()));
}

fn solve(input: &str) -> usize {
    let matrix = parse_char_matrix(input);

    matrix.columns().into_iter().map(solve_column).sum()
}

fn solve_column(col: ArrayView1<char>) -> usize {
    // println!("\ncol {col}");
    let cube_rock_positions = col.iter().positions(|c| *c == '#').map(|i| i + 1);
    let north_wall = std::iter::once(0);
    let south_wall = std::iter::once(col.len() + 1);
    let sum = north_wall
        .chain(cube_rock_positions)
        .chain(south_wall)
        .tuple_windows()
        .map(|(start, stop)| {
            let sub_col = col.slice(s![(start..stop - 1)]);
            // println!("sub_col {sub_col}");
            let sum: usize = sub_col
                .iter()
                .filter(|c| **c == 'O')
                .enumerate()
                .map(|(i, _)| col.len() - start - i)
                .sum();

            // print!("sum: {sum} ");
            sum
        })
        .sum();

    sum
}

#[test]
fn test_example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(solve(input), 136)
}
