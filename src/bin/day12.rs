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

    let Some(ans) = solve_recursive(&sizes, &condition.chars().collect_vec()) else {
        panic!("{input}")
    };
    ans
    // todo!()
}

fn solve_recursive(sizes: &[usize], conditions: &[char]) -> Option<usize> {
    if sizes.is_empty() {
        if conditions.contains(&'#') {
            return None;
        } else {
            // println!("{}", ".".repeat(conditions.len()));
            return Some(1);
        }
    } else if !(conditions.contains(&'#') || conditions.contains(&'?')) {
        return None;
    }
    let (current_size, remaining_sizes) = sizes.split_first().unwrap();
    if conditions.len() < *current_size {
        return None;
    }
    match conditions.first() {
        Some('.') => {
            // print!(".");
            return solve_recursive(sizes, &conditions[1..]);
        }
        Some('#') => {
            if conditions[..*current_size].contains(&'.') {
                return None;
            }
            match conditions.get(*current_size) {
                Some('#') => return None, // Not valid if next char is #
                None => {
                    // Last possible pos,
                    if conditions[..*current_size].contains(&'.') {
                        return None;
                    } else if remaining_sizes.is_empty() {
                        // println!("{}", "#".repeat(*current_size));
                        return Some(1);
                    } else {
                        return None;
                    }
                }
                _ => {
                    // print!("{}.", "#".repeat(*current_size));
                    return solve_recursive(remaining_sizes, &conditions[1 + *current_size..]);
                }
            }
        }
        Some('?') => {
            let mut sum_possible = 0;

            match conditions.get(*current_size) {
                Some('#') => {
                    // Not a valid spot, move right
                    // print!(".");
                    return solve_recursive(sizes, &conditions[1..]);
                }
                // Some('.') => {
                //     // Valid spot, but cannot move further right
                //     return solve_recursive(remaining_sizes, &conditions[1 + *current_size..]);
                // }
                Some(_) => {
                    if conditions[..*current_size].contains(&'.') {
                        // Not a valid spot, move right. NOTE: Move multi steps right
                        return solve_recursive(sizes, &conditions[1..]);
                    } else {
                        // print!("{}.", "#".repeat(*current_size));
                        let configs_for_this_pos =
                            solve_recursive(remaining_sizes, &conditions[1 + *current_size..])
                                .unwrap_or(0);
                        // dbg!(configs_for_this_pos);
                        // Step forward
                        // print!("-");
                        let configs_for_rest =
                            solve_recursive(sizes, &conditions[1..]).unwrap_or(0);
                        // dbg!(configs_for_rest);
                        return Some(configs_for_this_pos + configs_for_rest);
                    }
                }
                None => {
                    // Last possible pos,
                    if conditions[..*current_size].contains(&'.') {
                        return None;
                    } else if remaining_sizes.is_empty() {
                        // println!("{}", "#".repeat(*current_size));
                        return Some(1);
                    } else {
                        return None;
                    }
                }
                _ => {
                    panic!()
                }
            }
        }

        None => return None,
        _ => {}
    }

    todo!()
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
