#![allow(unused)]
use advent_of_code::{get_input, parse_char_matrix};
use itertools::Itertools;
use ndarray::prelude::*;

fn main() {
    println!("Answer: {}", solve(&get_input()));
}

fn solve(input: &str) -> usize {
    let mut matrix = parse_char_matrix(input);

    matrix.axis_iter_mut(Axis(1)).for_each(lean_north);
    calc_load(&matrix)
}

fn lean_north(mut col: ArrayViewMut1<char>) {
    let mut start = 0;

    let slices = col
        .iter()
        .positions(|c| *c == '#')
        .chain(std::iter::once(col.len()))
        .map(|stop| {
            let slice = s![(start..stop)];
            start = stop + 1;
            slice
        })
        .collect_vec();

    for slice in slices {
        let sub_col = col.slice_mut(slice);
        let num_rolling_stones = sub_col.iter().filter(|c| **c == 'O').count();
        let (mut filled, mut empty) = sub_col.split_at(Axis(0), num_rolling_stones);
        filled.fill('O');
        empty.fill('.');
    }
}

fn calc_load(matrix: &ArrayBase<ndarray::OwnedRepr<char>, Dim<[usize; 2]>>) -> usize {
    matrix.columns().into_iter().map(calc_load_column).sum()
}

fn calc_load_column(col: ArrayView1<char>) -> usize {
    let len = col.len();
    col.iter()
        .enumerate()
        .filter_map(|(i, c)| if *c == 'O' { Some(col.len() - i) } else { None })
        .sum()
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
