#![allow(unused)]
use advent_of_code::get_input;
use itertools::Itertools;

fn main() {
    println!("Answer: {}", solve(&get_input()));
}

#[derive(Clone)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

impl<'a> std::fmt::Debug for Lens<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

fn solve(input: &str) -> usize {
    let mut boxes = vec![vec![]; 256];
    for instruction in input.split(',') {
        if let Some((label, focal_length)) = instruction.split_once('=') {
            let slots = &mut boxes[get_hash(label)];
            let lens = Lens {
                focal_length: focal_length.parse().unwrap(),
                label,
            };
            if let Some(conflict) = slots
                .iter_mut()
                .find(|lens: &&mut Lens<'_>| lens.label == label)
            {
                *conflict = lens
            } else {
                slots.push(lens)
            };
        } else if let Some(label) = instruction.strip_suffix('-') {
            let slots = &mut boxes[get_hash(label)];
            if let Some(i) = slots.iter().position(|lens| lens.label == label) {
                let _ = slots.remove(i);
            }
        } else {
            panic!()
        }
        // dbg!(instruction);
        // for (i, b) in boxes.iter().enumerate().filter(|(_, b)| !b.is_empty()) {
        //     println!("Box {}: {b:?}", i);
        // }
        // println!();
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, slots)| {
            (i + 1)
                * slots
                    .iter()
                    .enumerate()
                    .map(|(j, lens)| (j + 1) * lens.focal_length)
                    .sum::<usize>()
        })
        .sum()
}

fn get_hash(word: &str) -> usize {
    word.chars()
        .fold(0u32, |acc, c| (acc + c as u32) * 17 % 256) as usize
}

#[test]
fn test_example() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(solve(input), 145)
}
