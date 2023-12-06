#![allow(unused)]

fn main() {
    let time = 47986698;
    let distance = 400121310111540;
    println!("Answer 1: {}", solve(time, distance));
    // println!("Answer 2: {}", solve_2(&input));
}

fn solve(t: u32, d: u64) -> usize {
    let t = t as f64;
    let d = d as f64;

    let max_x = t / 2.0 + (t * t / 4.0 - d).sqrt();
    let max_disc_x = if max_x % 1.0 == 0.0 {
        (max_x - 1.0) as u64
    } else {
        max_x.floor() as u64
    };

    let half_dist = (max_disc_x as f64 - t / 2.0);
    let sol = if half_dist % 1.0 == 0.0 {
        half_dist * 2.0 + 1.0
    } else {
        half_dist.ceil() * 2.0
    };
    dbg!(sol);
    sol as usize
}

#[test]
fn test_example() {
    let time = 71530;
    let distance = 940200;

    assert_eq!(solve(time, distance), 71503)
}
