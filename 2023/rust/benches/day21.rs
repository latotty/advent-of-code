use aoc2023::internal::day21::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day21.task");
    let result1 = process1(input);
    assert_eq!(result1, 3689);
}

// #[divan::bench]
// fn part2() {
//     let input = include_str!("../data/day21.task");
//     let result1 = process2(input);
//     assert_eq!(result1, 127517902575337);
// }
