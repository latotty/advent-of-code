use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day9.task").unwrap();

    let result1 = process1(&input);

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}

fn process1(input: &str) -> i64 {
    input.lines().map(process_line1).sum()
}

fn process_line1(line: &str) -> i64 {
    let mut last_nums: Vec<i64> = vec![];

    let mut nums = line
        .split(' ')
        .map(|n| str::parse::<i64>(n).unwrap_or_else(|_e| panic!("not a number {n}")))
        .collect::<Vec<i64>>();

    loop {
        last_nums.push(*nums.last().unwrap());
        nums = nums.windows(2).map(|ab| ab[1] - ab[0]).collect();

        if nums.iter().all(|n| *n == 0) {
            break;
        }
    }
    last_nums.iter().sum()
}

fn process2(input: &str) -> i64 {
    input.lines().map(process_line2).sum()
}

fn process_line2(line: &str) -> i64 {
    let mut first_nums: Vec<i64> = vec![];

    let mut nums = line
        .split(' ')
        .map(|n| str::parse::<i64>(n).unwrap_or_else(|_e| panic!("not a number {n}")))
        .collect::<Vec<i64>>();

    loop {
        first_nums.push(*nums.first().unwrap());
        nums = nums.windows(2).map(|ab| ab[1] - ab[0]).collect();

        if nums.iter().all(|n| *n == 0) {
            break;
        }
    }
    first_nums.iter().rev().fold(0, |acc, n| n - acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::c1("0 3 6 9 12 15", 18)]
    #[case::c2("1 3 6 10 15 21", 28)]
    #[case::c3("10 13 16 21 30 45", 68)]
    fn process_line1_test(#[case] line: &str, #[case] expected: i64) {
        let result = process_line1(line);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_process1_example1() {
        let input = fs::read_to_string("./data/day9.example1").unwrap();

        let result = process1(&input);

        assert_eq!(result, 114);
    }

    #[rstest]
    #[case::c1("0 3 6 9 12 15", -3)]
    #[case::c2("1 3 6 10 15 21", 0)]
    #[case::c3("10 13 16 21 30 45", 5)]
    fn process_line2_test(#[case] line: &str, #[case] expected: i64) {
        let result = process_line2(line);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_process2_example1() {
        let input = fs::read_to_string("./data/day9.example1").unwrap();

        let result = process2(&input);

        assert_eq!(result, 2);
    }
}
