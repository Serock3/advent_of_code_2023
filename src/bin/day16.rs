#![allow(unused)]
use std::collections::HashSet;
use std::f32::consts::E;
use std::ops::Add;

use advent_of_code::Pos;
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
    println!("Answer1: {}", solve1(&get_input()));
    println!("Answer2: {}", solve2(&get_input()));
}



#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Beam {
    start: Pos<usize>,
    dir: Direction,
}

fn solve1(input: &str) -> usize {
    let initial_beam = Beam {
        start: Pos(0, 0),
        dir: East,
    };
    let matrix = advent_of_code::parse_char_matrix(input);
    solve_initial(&matrix, initial_beam)
}

fn solve2(input: &str) -> usize {
    let matrix = advent_of_code::parse_char_matrix(input);
    let (len_row, len_col) = matrix.dim();

    let left_side = (0..len_row).map(|row| Beam {
        start: Pos(row, 0),
        dir: East,
    });
    let right_side = (0..len_row).map(|row| Beam {
        start: Pos(row, len_col-1),
        dir: West,
    });
    let upper_side = (0..len_col).map(|col| Beam {
        start: Pos(0, col),
        dir: South,
    });
    let lower_side = (0..len_col).map(|col| Beam {
        start: Pos(len_row-1, col),
        dir: North,
    });

    left_side.chain(right_side).chain(upper_side).chain(lower_side).map(|initial_beam|{
        solve_initial(&matrix, initial_beam)
    }).max().unwrap()
}

fn solve_initial(matrix: &Array2<char>, initial_beam: Beam) -> usize {
    let mut energized = Array2::from_elem(matrix.raw_dim(), false);

    let mut visited = HashSet::new();

    let mut queue = vec![initial_beam];

    while let Some(inbound_beam) = queue.pop() {
        // dbg!(&inbound_beam);
        if !visited.insert(inbound_beam.clone()) {
            continue;
        }
        let (maybe_reflections, beam_area) = handle_beam(&inbound_beam, matrix);
        if let Some(reflections) = maybe_reflections {
            queue.extend(reflections);
        }
        energized.slice_mut(beam_area).fill(true);
    }
    // for row in energized.rows(){
    //     for c in row.map(|b|if *b {'#'}else{'.'}){
    //         print!("{c}")
    //     }
    //     println!()
    // }
    energized.iter().filter(|heated| **heated).count()
}

#[allow(clippy::type_complexity)]
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
            let mut beam_area = s![..=pos.0; -1, pos.1];

            let maybe_new_beams = matrix
                .slice(beam_area)
                .iter()
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
            let mut beam_area = s![pos.0, ..=pos.1; -1];

            let maybe_new_beams = matrix
                .slice(beam_area)
                .iter()
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
        start: mirror_pos + Pos::from(&dir),
        dir,
    })
}

fn get_reflection_dir(inbound_dir: Direction, c: char) -> &'static [Direction] {
    match c {
        '/' => match inbound_dir {
            North => &[East],
            East => &[North],
            South => &[West],
            West => &[South],
        },
        '\\' => match inbound_dir {
            North => &[West],
            East => &[South],
            South => &[East],
            West => &[North],
        },
        '|' => match inbound_dir {
            North => &[North],
            East => &[North, South],
            South => &[South],
            West => &[North, South],
        },
        '-' => match inbound_dir {
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
        start: Pos(5, 4)
    },
    s![6, 0..=4])]
    #[case(Beam {
        start: Pos(0, 4),
        dir: South,
    }, Beam {
        dir: East,
        start: Pos(1, 5)
    },
    s![0..=1, 4])]
    #[case(Beam {
        start: Pos(9, 0),
        dir: East,
    }, Beam {
        dir: North,
        start: Pos(8, 2)
    },
    s![9, 0..=2])]
    #[case(Beam {
        dir: North,
        start: Pos(9, 8),
    }, Beam {
        dir: North,
        start: Pos(2, 8)
    },
    s![3..=9, 8])]
    #[case(Beam {
        dir: West,
        start: Pos(1, 9),
    }, Beam {
        dir: North,
        start: Pos(0, 4)
    },
    s![1, 4..=9])]
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
        start: Pos(1, 1),
    }, 
    Beam {
        dir: West,
        start: Pos(7, 0),
    },
    Beam {
        dir: East,
        start: Pos(7, 2),
    },
    s![1..=7, 1])]
    pub(crate) fn test_flat_reflection(
        #[case] inbound_beam: Beam,
        #[case] reflected_beam1: Beam,
        #[case] reflected_beam2: Beam,
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
        assert_eq!(refl, reflected_beam1);
        let refl = new_beams.next().unwrap();
        assert_eq!(refl, reflected_beam2);
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
        assert_eq!(solve1(input), 46)
    }

    #[test]
    pub(crate) fn test_example2() {
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
        assert_eq!(solve2(input), 51)
    }
}
