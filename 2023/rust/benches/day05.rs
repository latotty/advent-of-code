use aoc2023::internal::day05::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day5.task");
    let data = input.parse::<ParsedInput>().expect("should parse");
    let result1 = process1(&data);
    assert_eq!(result1, 621354867);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day5.task");
    let data = input.parse::<ParsedInput>().expect("should parse");
    let result1 = process2(&data);
    assert_eq!(result1, 15880236);
}
