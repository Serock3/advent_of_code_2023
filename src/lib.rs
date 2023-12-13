use std::panic::Location;

#[track_caller]
pub fn get_input() -> String {
    let n = Location::caller()
        .file()
        .strip_prefix("src/bin/day")
        .unwrap()
        .split_once(['_', '.', '-', 'p'])
        .unwrap()
        .0;
    let input_path = format!("input/day{n}.txt");
    std::fs::read_to_string(input_path).unwrap()
}
