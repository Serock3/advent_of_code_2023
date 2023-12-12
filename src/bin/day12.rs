#![allow(unused)]
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();
    println!("Answer: {}", solve(&input));
}

fn solve(input: &str) -> usize {
    input.lines().map(solve_single).sum()
}

fn solve_single(input: &str) -> usize {
    let (condition, sizes) = input.split_once(' ').unwrap();
    let sizes = sizes
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect_vec();

    // let contiguous_interval = condition
    //     .split('.')
    //     .filter(|s| !s.is_empty())
    //     // .map(|contiguous_working| contiguous_working.len())
    //     .collect_vec();

    solve_recursive(&sizes, &condition.chars().collect_vec())
}

fn solve_recursive(remaining_sizes: &[usize], remaning_conditions: &[char]) -> usize {
    let Some((current_size, remaining_sizes)) = remaining_sizes.split_first() else {
        return 1;
    };
    let mut sum_possible = 0;
    for i in 0..(remaning_conditions.len() - current_size) {
        match remaning_conditions.get(i + *current_size) {
            Some('#') => continue, // Not valid if next char is #
            None => return 0,
            _ => {}
        }
        let window = &remaning_conditions[i..i + *current_size];
        if window.contains(&'.') {
            continue;
        }
        if let Some(next_win) = remaning_conditions.get(i + *current_size + 1..) {
            sum_possible += solve_recursive(remaining_sizes, next_win)
        } else {
            return sum_possible + 1;
        }
    }

    sum_possible
}

mod tests {
    use super::*;

    #[test]
    pub(crate) fn test_example_single() {
        let input = "???.### 1,1,3";
        assert_eq!(solve(input), 1)
    }

    #[test]
    pub(crate) fn test_example_single2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(solve(input), 4)
    }

    #[test]
    pub(crate) fn test_example_single3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(solve(input), 1)
    }

    #[test]
    pub(crate) fn test_example_single4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(solve(input), 1)
    }

    #[test]
    pub(crate) fn test_example_single5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(solve(input), 4)
    }
    #[test]
    pub(crate) fn test_example_single6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(solve(input), 10)
    }

    #[test]
    pub(crate) fn test_example_single_custom1() {
        let input = "??#.?#?#??? 1,3,1";
        assert_eq!(solve(input), 2)
    }

    #[test]
    pub(crate) fn test_example_single_custom2() {
        let input = "?????.??.???. 1,1,1";
        assert_eq!(solve(input), 2)
    }

    #[test]
    pub(crate) fn test_example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(solve(input), 21)
    }
}
