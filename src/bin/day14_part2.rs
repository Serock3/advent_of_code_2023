#![allow(unused)]
use advent_of_code::{get_input, parse_char_matrix};
use itertools::Itertools;
use ndarray::prelude::*;
use rayon::prelude::*;

fn main() {
    println!("Answer: {}", solve(&get_input()));
}

fn solve(input: &str) -> usize {
    let mut matrix = parse_char_matrix(input);

    let mut history: Vec<Array2<char>> = Vec::with_capacity(128);
    loop {
        spin(&mut matrix);
        // dbg!(&matrix);
        for (i, prev_m) in history.iter().enumerate() {
            if matrix == prev_m {
                let looping_sequence = &history[i..];
                let remaining_spins = 1_000_000_000 - history.len() - 1;
                let stop_index = remaining_spins % looping_sequence.len();

                return calc_load(&looping_sequence[stop_index]);
            }
        }
        history.push(matrix.clone());
    }
}

fn spin(matrix: &mut Array2<char>) {
    matrix
        .axis_iter_mut(Axis(1))
        .for_each(|col| lean(col, false));
    matrix
        .axis_iter_mut(Axis(0))
        .for_each(|row| lean(row, false));
    matrix
        .axis_iter_mut(Axis(1))
        .for_each(|col| lean(col, true));
    matrix
        .axis_iter_mut(Axis(0))
        .for_each(|row| lean(row, true));
}

fn lean(mut col: ArrayViewMut1<char>, backwards: bool) {
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
        let len = sub_col.len();
        let num_rolling_stones = sub_col.iter().filter(|c| **c == 'O').count();
        let (mut filled, mut empty) = {
            if backwards {
                let tuple = sub_col.split_at(Axis(0), len - num_rolling_stones);
                (tuple.1, tuple.0)
            } else {
                sub_col.split_at(Axis(0), num_rolling_stones)
            }
        };
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
    assert_eq!(solve(input), 64)
}
