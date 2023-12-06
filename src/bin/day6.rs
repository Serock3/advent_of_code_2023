#![allow(unused)]

fn main() {
    let time = [47, 98, 66, 98];
    let distance = [400, 1213, 1011, 1540];
    println!("Answer 1: {}", solve(&time, &distance));
    // println!("Answer 2: {}", solve_2(&input));
}

fn solve(time: &[i32], distance: &[i32]) -> usize {
    time.iter()
        .zip(distance)
        .map(|(t, d)| {
            let t = *t as f32;
            let d = *d as f32;

            let max_x = t / 2.0 + (t * t / 4.0 - d).sqrt();
            let max_disc_x = if max_x % 1.0 == 0.0 {
                (max_x - 1.0) as u32
            } else {
                max_x.floor() as u32
            };

            let half_dist = (max_disc_x as f32 - t / 2.0);
            let sol = if half_dist % 1.0 == 0.0 {
                half_dist * 2.0 + 1.0
            } else {
                half_dist.ceil() * 2.0
            };
            dbg!(sol);
            sol as usize
        })
        .product()
}

#[test]
fn test_example() {
    let time = [7, 15, 30];
    let distance = [9, 40, 200];

    assert_eq!(solve(&time, &distance), 288)
}
