#![allow(unused)]
use std::collections::HashSet;
use std::f32::consts::E;
use std::ops::Add;

use advent_of_code::get_input;
use advent_of_code::Direction;
use advent_of_code::Direction::*;
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::s;
use ndarray::Array2;
use ndarray::AssignElem;
use ndarray::Axis;
use ndarray::SliceInfo;
use ndarray::SliceInfoElem;

fn main() {
    println!("Answer: {}", solve(&get_input()));
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Pos<T>(T, T);

impl<T: Add<Output = T>> Add for Pos<T> {
    type Output = Pos<T>;

    fn add(self, rhs: Pos<T>) -> Pos<T> {
        Pos(self.0 + rhs.0, self.1 +rhs.1)
    }
}

impl Add<Pos<isize>> for Pos<usize> {
    type Output = Pos<usize>;

    fn add(self, rhs: Pos<isize>) -> Pos<usize> {
        Pos(self.0.saturating_add_signed(rhs.0) , self.1.saturating_add_signed(rhs.1) )
    }
}

impl From<&Direction> for Pos<isize> {
    fn from(value: &Direction) -> Self {
        match value {
            North => Pos(-1, 0),
            East => Pos(0, 1),
            South => Pos(1, 0),
            West => Pos(0, -1),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Beam {
    start: Pos<usize>,
    dir: Direction,
}

fn solve(input: &str) -> usize {
    let matrix = advent_of_code::parse_char_matrix(input);
    let mut energized = Array2::from_elem(matrix.raw_dim(), false);

    let mut visited = HashSet::new();

    let mut queue = vec![Beam {
        start: Pos(0, 0),
        dir: East,
    }];

    while let Some(inbound_beam) = queue.pop() {
        // dbg!(&inbound_beam);
        if !visited.insert(inbound_beam.clone()) {
            continue;
        }
        let (maybe_reflections, beam_area) = handle_beam(&inbound_beam, &matrix);
        if let Some(reflections) = maybe_reflections {
            queue.extend(reflections);
        }
        energized.slice_mut(beam_area).fill(true);
    }
    for row in energized.rows(){
        for c in row.map(|b|if *b {'#'}else{'.'}){
            print!("{c}")
        }
        println!()
    }
    energized.iter().filter(|heated| **heated).count()
}

fn handle_beam(
    inbound_beam: &Beam,
    matrix: &Array2<char>,
) -> (
    Option<impl Iterator<Item = Beam>>,
    SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 1]>>,
) {
    let pos = &inbound_beam.start;
    match inbound_beam.dir {
        North => {
            let mut beam_area = s![..=pos.0, pos.1];

            let maybe_new_beams = matrix
                .slice(beam_area)
                .iter()
                .rev()
                .find_position(|&&c| c != '.')
                .map(|(offset, c)| {
                    
                    let start_row = pos.0 - offset;
                    beam_area = s![start_row..=pos.0, pos.1];
                    let mirror_pos = Pos(start_row, pos.1);

                    new_beams(inbound_beam.dir, c, mirror_pos)
                });
            (maybe_new_beams, beam_area)
        }
        East => {
            let mut beam_area = s![pos.0, pos.1..];
            let maybe_new_beams = matrix
                .slice(beam_area)
                .iter()
                .find_position(|&&c| c != '.')
                .map(|(offset, c)| {
                    let end_col = pos.1 + offset;
                    beam_area = s![pos.0, pos.1..=end_col];
                    let mirror_pos = Pos(pos.0, end_col);

                    new_beams(inbound_beam.dir, c, mirror_pos)
                });
            (maybe_new_beams, beam_area)
        }
        South => {
            let mut beam_area = s![pos.0.., pos.1];

            let maybe_new_beams = matrix
                .slice(beam_area)
                .iter()
                .find_position(|&&c| c != '.')
                .map(|(offset, c)| {
                    let end_row = pos.0 + offset;
                    beam_area = s![pos.0..=end_row, pos.1];
                    let mirror_pos = Pos(end_row, pos.1);

                    new_beams(inbound_beam.dir, c, mirror_pos)
                });
            (maybe_new_beams, beam_area)
        }
        West => {
            let mut beam_area = s![pos.0, ..=pos.1];

            let maybe_new_beams = matrix
                .slice(beam_area)
                .iter()
                .rev()
                .find_position(|&&c| c != '.')
                .map(|(offset, c)| {
                    let start_col = pos.1 - offset;
                    beam_area = s![pos.0, start_col..=pos.1];
                    let mirror_pos = Pos(pos.0, start_col);

                    new_beams(inbound_beam.dir, c, mirror_pos)
                });
            (maybe_new_beams, beam_area)
        }
    }
}

fn new_beams(
    inbound_dir: Direction,
    c: &char,
    mirror_pos: Pos<usize>,
) -> impl Iterator<Item = Beam> {
    let next_dirs: &[Direction] = get_reflection_dir(inbound_dir, *c);
    next_dirs.iter().cloned().map(move |dir| Beam {
        start: mirror_pos.clone() + Pos::from(&dir),
        dir,
    })
}

fn get_reflection_dir(inbond_dir: Direction, c: char) -> &'static [Direction] {
    match c {
        '/' => match inbond_dir {
            North => &[East],
            East => &[North],
            South => &[West],
            West => &[South],
        },
        '\\' => match inbond_dir {
            North => &[West],
            East => &[South],
            South => &[East],
            West => &[North],
        },
        '|' => match inbond_dir {
            North => &[North],
            East => &[North, South],
            South => &[South],
            West => &[North, South],
        },
        '-' => match inbond_dir {
            North => &[West, East],
            East => &[East],
            South => &[West, East],
            West => &[West],
        },
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Beam {
        start: Pos(6, 0),
        dir: East,
    }, Beam {
        dir: North,
        start: Pos(6, 4)
    },
    s![6, 0..4])]
    #[case(Beam {
        start: Pos(0, 4),
        dir: South,
    }, Beam {
        dir: East,
        start: Pos(1, 4)
    },
    s![0..1, 4])]
    #[case(Beam {
        start: Pos(9, 0),
        dir: East,
    }, Beam {
        dir: North,
        start: Pos(9, 2)
    },
    s![9, 0..2])]
    #[case(Beam {
        dir: North,
        start: Pos(9, 8),
    }, Beam {
        dir: North,
        start: Pos(3, 8)
    },
    s![3..9, 8])]
    #[case(Beam {
        dir: West,
        start: Pos(1, 9),
    }, Beam {
        dir: North,
        start: Pos(1, 4)
    },
    s![1, 4..9])]
    pub(crate) fn test_diag_reflection(
        #[case] inbound_beam: Beam,
        #[case] reflected_beam: Beam,
        #[case] beam_area: SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 1]>>,
    ) {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let matrix = advent_of_code::parse_char_matrix(input);

        let (maybe_new_beams, new_beam_area) = handle_beam(&inbound_beam, &matrix);
        assert_eq!(*new_beam_area, *beam_area);
        let new_beams = &mut maybe_new_beams.unwrap();
        let refl = new_beams.next().unwrap();
        assert_eq!(refl, reflected_beam);
        assert_eq!(new_beams.next(), None);
    }

    #[rstest]
    #[case(Beam {
        dir: South,
        start: Pos(0, 1),
    }, 
    &[West, East],
    Pos(7, 1),
    s![0..7, 1])]
    pub(crate) fn test_flat_reflection(
        #[case] inbound_beam: Beam,
        #[case] out_dirs: &'static [Direction],
        #[case] end_pos: Pos<usize>,
        #[case] beam_area: SliceInfo<[SliceInfoElem; 2], Dim<[usize; 2]>, Dim<[usize; 1]>>,
    ) {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let matrix = advent_of_code::parse_char_matrix(input);

        let (maybe_new_beams, new_beam_area) = handle_beam(&inbound_beam, &matrix);
        assert_eq!(*new_beam_area, *beam_area);
        let new_beams = &mut maybe_new_beams.unwrap();
        let refl = new_beams.next().unwrap();
        assert_eq!(refl, Beam{dir: out_dirs[0],start: end_pos.clone()});
        let refl = new_beams.next().unwrap();
        assert_eq!(refl, Beam{dir: out_dirs[1],start: end_pos});
        assert_eq!(new_beams.next(), None);
    }

    #[test]
    pub(crate) fn test_example() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(solve(input), 46)
    }
}
