use aoc2023::internal::day04::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day4.task");
    let result1 = process1(input);
    assert_eq!(result1, 26914);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day4.task");
    let result1 = process2(input);
    assert_eq!(result1, 13080971);
}
