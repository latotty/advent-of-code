use aoc2023::internal::day05::*;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day5.task").unwrap();
    let data = input.parse::<ParsedInput>().expect("should parse");

    let result1 = process1(&data.clone());

    println!("Result1: {result1}");

    let result2 = process2(&data);

    println!("Result2: {result2}");
}
