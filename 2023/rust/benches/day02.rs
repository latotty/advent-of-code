use aoc2023::internal::day02::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day2.task");
    let result1 = process1(input);
    assert_eq!(result1, 2162);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day2.task");
    let result1 = process2(input);
    assert_eq!(result1, 72513);
}