use aoc2023::internal::day12::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day12.task");
    let result1 = process1(input);
    assert_eq!(result1, 7204);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day12.task");
    let result1 = process2(input);
    assert_eq!(result1, 1672318386674);
}
