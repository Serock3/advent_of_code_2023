#![allow(unused)]

use std::ops::Div;

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

use advent_of_code::{
    Direction::{self, *},
    Pos,
};

fn solve(input: &str) -> usize {
    let char_matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let s_pos = find_s(&char_matrix);

    let start_dirs = [North, East, South, West];
    let mut longest = 0;
    start_dirs
        .into_iter()
        .find_map(|dir| follow_pipe(s_pos, dir, &char_matrix))
        .unwrap()
}

fn find_s(char_matrix: &[Vec<char>]) -> Pos<isize> {
    char_matrix
        .iter()
        .enumerate()
        .find_map(|(r, row)| {
            row.iter().enumerate().find_map(|(c, character)| {
                if *character == 'S' {
                    Some(Pos(r.try_into().unwrap(), c.try_into().unwrap()))
                } else {
                    None
                }
            })
        })
        .unwrap()
}

fn follow_pipe(
    mut pos: Pos<isize>,
    mut dir: Direction,
    char_matrix: &[Vec<char>],
) -> Option<usize> {
    let start_dir = dir;

    // Twice the area enclosed by the looping pipes
    let mut signed_enclosed_area = 0;
    // To account for the area of the pipes, we need the number of them
    let mut num_pipes = 0;

    loop {
        signed_enclosed_area += twice_area_change(pos, &dir);

        pos = step(pos, &dir);
        let pipe = get_pipe(pos, char_matrix)?;

        num_pipes += 1;

        dir = match (dir, pipe) {
            (North, '|') => North,
            (North, 'F') => East,
            (North, '7') => West,
            (East, '-') => East,
            (East, '7') => South,
            (East, 'J') => North,
            (South, '|') => South,
            (South, 'J') => West,
            (South, 'L') => East,
            (West, '-') => West,
            (West, 'L') => North,
            (West, 'F') => South,
            (dir, 'S') => return Some(get_gap_area(signed_enclosed_area, num_pipes)),
            // Hit a stop
            _ => return None,
        };
    }
}

/// Get the total enclosed area by the pipes, accounting for their own thickness.
///
/// Almost half the pipes own area is inside the curve. A little less because the
/// inside perimeter of the pipes is smaller than the outside perimeter.
fn get_gap_area(signed_enclosed_area: isize, num_pipes: usize) -> usize {
    let unsigned_area: usize = signed_enclosed_area.abs().try_into().unwrap();
    unsigned_area - num_pipes / 2 + 1
}

/// Calculate the difference in enclosed area made by line element according
/// to Stoke's/Green's theorem. This will not include the thickness of the pipes,
/// which is corrected for by [`get_gap_area`].
///
/// See https://en.wikipedia.org/wiki/Green%27s_theorem#Area_calculation
fn twice_area_change(pos: Pos<isize>, dir: &Direction) -> isize {
    let x = pos.0;
    let dy = Pos::from(dir).1;
    x * dy
}

fn step(s_pos: Pos<isize>, dir: &Direction) -> Pos<isize> {
    let Pos(r, c) = Pos::from(dir);
    Pos(s_pos.0 + r, s_pos.1 + c)
}

/// Indexes into char matrix and gets pipe. Filters '.' chars.
fn get_pipe(pos: Pos<isize>, char_matrix: &[Vec<char>]) -> Option<char> {
    let row = usize::try_from(pos.0).ok()?;
    let col = usize::try_from(pos.1).ok()?;

    char_matrix
        .get(row)
        .and_then(|chars| chars.get(col))
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Hand made test for by gap area fn
    #[test]
    pub(crate) fn test_gap_area() {
        assert_eq!(get_gap_area(10, 16), 3)
    }

    #[test]
    pub(crate) fn test_gap_area2() {
        assert_eq!(get_gap_area(9, 16), 2)
    }

    #[test]
    pub(crate) fn test_example() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(solve(input), 4)
    }

    #[test]
    pub(crate) fn test_example2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(solve(input), 8)
    }

    #[test]
    pub(crate) fn test_example3() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(solve(input), 1)
    }
}
