#![allow(unused)]
use advent_of_code::{get_input, parse_char_matrix};
use itertools::Itertools;
use ndarray::prelude::*;

fn main() {
    println!("Answer: {}", solve(&get_input()));
}

fn solve(input: &str) -> usize {
    let matrix = parse_char_matrix(input);

    matrix.columns().into_iter().map(fun_name).sum()
}

fn fun_name(col: ArrayView1<char>) -> usize {
    let cube_rock_positions = col.iter().positions(|c| *c == '#').map(|i| i + 1);
    let north_wall = std::iter::once(0);
    let south_wall = std::iter::once(col.len());
    let sum = north_wall
        .chain(cube_rock_positions)
        .chain(south_wall)
        .tuples()
        .map(|(higher_cube_rock, lower_cube_rock)| {
            let x = col.slice(s![higher_cube_rock..lower_cube_rock - 1]);
            print!("{x} ");
            let sum: usize = x
                .iter()
                .filter(|c| **c == 'O')
                .enumerate()
                .map(|(i, _)| col.len() - higher_cube_rock - i)
                .sum();

            print!("sum: {sum} ");
            sum
        })
        .sum();
    println!();

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
