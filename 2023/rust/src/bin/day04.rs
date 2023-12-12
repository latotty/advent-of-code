use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day4.task").unwrap();

    let result1 = process1(&input);

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}

fn process1(input: &str) -> usize {
    let mut result = 0;

    for line in input.lines() {
        let winning_nums = get_winning_amount(line);

        if winning_nums > 0 {
            let amount = {
                let mut n = 1;
                for _ in 0..winning_nums - 1 {
                    n *= 2;
                }
                n
            };
            result += amount;
        }
    }

    result
}

fn process2(input: &str) -> usize {
    let mut result = 0;

    let mut winnings: VecDeque<usize> = VecDeque::from([]);

    for line in input.lines() {
        let winning_nums = get_winning_amount(line);
        let card_amount = 1 + winnings.pop_front().unwrap_or_default();
        result += card_amount;
        for idx in 0..winning_nums {
            if let Some(winning) = winnings.get_mut(idx) {
                *winning += card_amount;
            } else {
                winnings.push_back(card_amount);
            }
        }
    }

    result
}

fn get_winning_amount(line: &str) -> usize {
    let number_groups = line
        .split(": ")
        .nth(1)
        .unwrap()
        .split('|')
        .map(|group| {
            group
                .split(' ')
                .filter_map(|num_str| num_str.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    number_groups[1]
        .clone()
        .into_iter()
        .filter(|num| number_groups[0].contains(num))
        .collect::<Vec<usize>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    };

    #[test]
    fn test_example1() {
        let result = process1(EXAMPLE_1);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_example2() {
        let result = process2(EXAMPLE_1);

        assert_eq!(result, 30);
    }
}
