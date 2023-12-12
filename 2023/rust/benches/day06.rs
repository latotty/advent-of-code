use std::time::Duration;

use aoc2023::internal::day06::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day6.task");
    let result1 = process1(input);
    assert_eq!(result1, 500346);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day6.task");
    let result1 = process2(input);
    assert_eq!(result1, 42515755);
}

#[divan::bench(
    max_time = Duration::from_millis(200)
)]
fn part1_bruteforce() {
    let input = include_str!("../data/day6.task");
    let result1 = process1_bruteforce(input);
    assert_eq!(result1, 500346);
}

#[divan::bench(
    max_time = Duration::from_millis(200)
)]
fn part2_bruteforce() {
    let input = include_str!("../data/day6.task");
    let result1 = process2_bruteforce(input);
    assert_eq!(result1, 42515755);
}
