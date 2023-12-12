use aoc2023::internal::day11::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day11.task");
    let result1 = process1(input);
    assert_eq!(result1, 9274989);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day11.task");
    let result1 = process2(input);
    assert_eq!(result1, 357134560737);
}
