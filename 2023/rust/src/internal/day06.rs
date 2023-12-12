use std::iter::zip;

pub fn process1(input: &str) -> u64 {
    let mut lines = input
        .lines()
        .map(|line| line.split(' ').filter_map(|s| s.parse::<u64>().ok()));
    let input: Vec<(u64, u64)> = zip(
        lines.next().unwrap(),
        lines.next().unwrap(),
    )
    .collect();

    input
        .iter()
        .map(|(time, distance)| get_winnings_algebraic(*time, *distance))
        .product()
}
pub fn process1_bruteforce(input: &str) -> u64 {
    let mut lines = input
        .lines()
        .map(|line| line.split(' ').filter_map(|s| s.parse::<u64>().ok()));
    let input: Vec<(u64, u64)> = zip(
        lines.next().unwrap(),
        lines.next().unwrap(),
    )
    .collect();

    input
        .iter()
        .map(|(time, distance)| get_winnings_bruteforce(*time, *distance))
        .product()
}

pub fn process2(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .as_str()
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .as_str()
        .parse::<u64>()
        .unwrap();

    get_winnings_algebraic(time, distance)
}

pub fn process2_bruteforce(input: &str) -> u64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .as_str()
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .as_str()
        .parse::<u64>()
        .unwrap();

    get_winnings_bruteforce(time, distance)
}

pub fn get_winnings_bruteforce(time: u64, dist: u64) -> u64 {
    let mut winnings = 0;
    for push_time in (1..time - 1).rev() {
        let c_distance = push_time * (time - push_time);
        if c_distance > dist {
            winnings += 1;
        } else if winnings > 0 {
            break;
        }
    }
    winnings
}

pub fn get_winnings_algebraic(time: u64, dist: u64) -> u64 {
    const BIG_EPSILON: f64 = f64::EPSILON * 10.;
    let part = ((time.pow(2) - 4 * dist) as f64).sqrt() / 2.;
    let time_f = time as f64 / 2.;
    let num1 = (time_f - part + BIG_EPSILON + 1.).trunc() as u64;
    let num2 = (time_f + part - BIG_EPSILON).trunc() as u64;

    num2 - num1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    const EXAMPLE_1: &str = indoc! {
        "Time:      7  15   30
        Distance:  9  40  200"
    };

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn get_winnings_bruteforce_test(#[case] time: u64, #[case] dist: u64, #[case] expected: u64) {
        let result = get_winnings_bruteforce(time, dist);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn get_winnings_algebraic_test(#[case] time: u64, #[case] dist: u64, #[case] expected: u64) {
        let result = get_winnings_algebraic(time, dist);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example1() {
        let result = process1(EXAMPLE_1);

        assert_eq!(result, 288);
    }

    #[test]
    fn test_example2() {
        let result = process2(EXAMPLE_1);

        assert_eq!(result, 71503);
    }
}
