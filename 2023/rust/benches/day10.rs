use aoc2023::internal::day10::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day10.task");
    let result1 = process1(input);
    assert_eq!(result1, 7030);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day10.task");
    let result1 = process2(input);
    assert_eq!(result1, 285);
}
