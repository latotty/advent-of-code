use std::fs;
use aoc2023::internal::day01::*;

fn main() {
    let input = fs::read_to_string("./data/day1.task").unwrap();

    let result1 = process1(&input.clone());

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}
