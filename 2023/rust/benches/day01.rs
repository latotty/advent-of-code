use aoc2023::internal::day01::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day1.task");
    let result1 = process1(input);
    assert_eq!(result1, 56042);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day1.task");
    let result1 = process2(input);
    assert_eq!(result1, 55358);
}