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

    #[test]
    fn test_example1() {
        let input = fs::read_to_string("./data/day4.example1").unwrap();

        let result = process1(&input);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_example2() {
        let input = fs::read_to_string("./data/day4.example1").unwrap();

        let result = process2(&input);

        assert_eq!(result, 30);
    }
}
