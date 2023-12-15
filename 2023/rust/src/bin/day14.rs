use aoc2023::internal::day14::{task1::process1, task2::process2};
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day14.task").unwrap();

    let result1 = process1(&input);

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}
