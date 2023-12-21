use aoc2023::internal::day21::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1_simple() {
    let input = include_str!("../data/day21.task");
    let result1 = process1_simple(input);
    assert_eq!(result1, 3689);
}

#[divan::bench]
fn part1_par() {
    let input = include_str!("../data/day21.task");
    let result1 = process1_par(input);
    assert_eq!(result1, 3689);
}

// #[divan::bench]
// fn part2() {
//     let input = include_str!("../data/day21.task");
//     let result1 = process2(input);
//     assert_eq!(result1, 127517902575337);
// }
