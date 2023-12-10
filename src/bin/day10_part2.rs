#![allow(unused)]

use std::ops::Div;

use itertools::Itertools;
use num::Integer;

fn main() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}
use Direction::*;

type Pos = (i32, i32);

impl From<&Direction> for Pos {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

fn solve(input: &str) -> u32 {
    let s_pos_raw = input.find('S').unwrap();
    let char_matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let (mut s_row, mut s_col) = (0, 0);
    'outer: for (r, row) in char_matrix.iter().enumerate() {
        for (c, character) in row.iter().enumerate() {
            if *character == 'S' {
                s_row = r;
                s_col = c;
                break 'outer;
            }
        }
    }
    assert_eq!(char_matrix[s_row][s_col], 'S');

    let start_dirs = [North, East, South, West];
    let mut longest = 0;
    start_dirs
        .into_iter()
        .find_map(|dir| {
            follow_pipe(
                (s_row.try_into().unwrap(), s_col.try_into().unwrap()),
                dir,
                &char_matrix,
            )
        })
        .unwrap()
}

fn follow_pipe(mut pos: Pos, mut dir: Direction, char_matrix: &[Vec<char>]) -> Option<u32> {
    let start_dir = dir.clone();

    // Twice the area enclosed by the looping pipes
    let mut twice_signed_enclosed_erea = 0;
    // To account for the area of the pipes, we need the number of them
    let mut num_pipes = 0;

    loop {
        twice_signed_enclosed_erea += twise_area_change(pos, &dir);

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
            (dir, 'S') => return Some(get_gap_area(twice_signed_enclosed_erea / 2, num_pipes)),
            // Hit a stop
            _ => return None,
        };
    }
}

/// Get the total enclosed area by the pipes, encounting for their own thickness.
///
/// Almost half the pipes own area is inside the curve. A little less because the
/// inside perimiter of the pipes is smaller than the outside perimiter.
fn get_gap_area(signed_enclosed_area: i32, num_pipes: u32) -> u32 {
    let unsigned_area: u32 = signed_enclosed_area.abs().try_into().unwrap();
    unsigned_area - num_pipes / 2 + 1
}

/// Calculate the difference in enclosed area made by line element according
/// to Stoke's/Green's theorem. This will not include the thickness of the pipes,
/// which is corrected for by [`get_gap_area`].
///
/// See https://en.wikipedia.org/wiki/Green%27s_theorem#Area_calculation
fn twise_area_change(pos: Pos, dir: &Direction) -> i32 {
    let (dr, dc) = <Pos>::from(dir);
    let (dx, dy) = (dc, -dr);
    let (x, y) = (pos.1, -pos.0);
    x * dy - y * dx
}

fn step(s_pos: Pos, dir: &Direction) -> Pos {
    let (r, c) = <Pos>::from(dir);
    (s_pos.0 + r, s_pos.1 + c)
}

/// Indexes into char matrix and gets pipe. Filters '.' chars.
fn get_pipe(pos: Pos, char_matrix: &[Vec<char>]) -> Option<char> {
    let row = usize::try_from(pos.0).ok()?;
    let col = usize::try_from(pos.1).ok()?;

    char_matrix
        .get(row)
        .and_then(|chars| chars.get(col))
        .cloned()
        .filter(|char| *char != '.')
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
