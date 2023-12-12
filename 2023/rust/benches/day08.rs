use aoc2023::internal::day08::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day8.task");
    let result1 = process1(input);
    assert_eq!(result1, 19783);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day8.task");
    let result1 = process2(input);
    assert_eq!(result1, 9177460370549);
}
