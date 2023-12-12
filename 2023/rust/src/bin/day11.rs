use aoc2023::internal::day11::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day11.task").unwrap();

    let result1 = process1(&input);

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}
