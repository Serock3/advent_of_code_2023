#![allow(unused)]
use std::collections::HashMap;

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

    let mut cache = HashMap::new();
    solve_recursive(&sizes, &condition.chars().collect_vec(), &mut cache)
}

fn solve_recursive<'a>(
    sizes: &'a [usize],
    conditions: &'a [char],
    cache: &mut HashMap<(&'a [usize], &'a [char]), usize>,
) -> usize {
    if sizes.is_empty() {
        return !conditions.contains(&'#') as usize;
    }
    if !(conditions.contains(&'#') || conditions.contains(&'?')) {
        return 0;
    }
    if conditions.len() < sizes.iter().sum::<usize>() + sizes.len() - 1 {
        return 0;
    }

    if let Some(chached_ret) = cache.get(&(sizes, conditions)) {
        return *chached_ret;
    }

    let (current_size, remaining_sizes) = sizes.split_first().unwrap();
    let ret = match conditions[0] {
        '.' => solve_recursive(sizes, &conditions[1..], cache),
        '#' => {
            if conditions[..*current_size].contains(&'.') {
                0
            } else {
                match conditions.get(*current_size) {
                    Some('#') => 0, // Not valid if next char is #
                    None => remaining_sizes.is_empty() as usize,
                    _ => solve_recursive(remaining_sizes, &conditions[1 + *current_size..], cache),
                }
            }
        }
        _ => {
            match conditions.get(*current_size) {
                Some('#') => {
                    // Not a valid spot, move right
                    solve_recursive(sizes, &conditions[1..], cache)
                }
                None => {
                    // Last possible pos,
                    (!conditions[..*current_size].contains(&'.') && remaining_sizes.is_empty())
                        as usize
                }
                _ => {
                    if conditions[..*current_size].contains(&'.') {
                        // Not a valid spot, move right. NOTE: Move multi steps right
                        solve_recursive(sizes, &conditions[1..], cache)
                    } else {
                        let configs_for_this_pos = solve_recursive(
                            remaining_sizes,
                            &conditions[1 + *current_size..],
                            cache,
                        );
                        let configs_for_rest = solve_recursive(sizes, &conditions[1..], cache);

                        configs_for_this_pos + configs_for_rest
                    }
                }
            }
        }
    };
    cache.insert((sizes, conditions), ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub(crate) fn test_simple() {
        let input = "###.### 3,3";
        assert_eq!(solve_single(input), 1)
    }

    #[test]
    pub(crate) fn test_simple2() {
        let input = "###.?## 3,3";
        assert_eq!(solve_single(input), 1)
    }

    #[test]
    pub(crate) fn test_example_single() {
        let input = "???.### 1,1,3";
        assert_eq!(solve_single(input), 1)
    }

    #[test]
    pub(crate) fn test_example_single2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(solve_single(input), 4)
    }

    #[test]
    pub(crate) fn test_example_single3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(solve_single(input), 1)
    }

    #[test]
    pub(crate) fn test_example_single4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(solve_single(input), 1)
    }

    #[test]
    pub(crate) fn test_example_single5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(solve_single(input), 4)
    }
    #[test]
    pub(crate) fn test_example_single6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(solve_single(input), 10)
    }

    #[test]
    pub(crate) fn test_example_single_custom1() {
        let input = "??#.?#?#??? 1,3,1";
        assert_eq!(solve_single(input), 2)
    }

    #[test]
    pub(crate) fn test_example_single_custom2() {
        let input = "?.#?#??#?# 1,6";
        assert_eq!(solve_single(input), 1)
    }

    #[test]
    pub(crate) fn test_example_single_custom3() {
        let input = ".??#???.??? 3,1,1";
        assert_eq!(solve_single(input), 12)
    }

    #[test]
    pub(crate) fn test_example_single_custom4() {
        let input = "??##?#?????.. 5,1";
        assert_eq!(solve_single(input), 7)
    }

    #[test]
    pub(crate) fn test_example_single_custom5() {
        let input = "?#?#?????. 1,1,2";
        assert_eq!(solve_single(input), 3)
    }

    #[test]
    pub(crate) fn test_example_single_custom6() {
        let input = "??.??#.??#?? 1,3,2,1";
        assert_eq!(solve_single(input), 2)
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
