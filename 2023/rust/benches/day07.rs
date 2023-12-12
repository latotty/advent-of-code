use aoc2023::internal::day07::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day7.task");
    let result1 = process1(input);
    assert_eq!(result1, 248217452);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day7.task");
    let result1 = process2(input);
    assert_eq!(result1, 245576185);
}
