use std::{cmp::Ordering, collections::HashMap};

const CARD_ORDER_1: &str = "AKQJT98765432";
const CARD_ORDER_2: &str = "AKQT98765432J";

pub fn process1(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<u64>().expect("should be u64"),
            )
        })
        .map(|(cards, bid)| {
            (
                cards,
                bid,
                get_cards_strength(cards, false),
            )
        })
        .collect::<Vec<(&str, u64, u8)>>();
    hands.sort_by(|a, b| sort_hands(a, b, CARD_ORDER_1));
    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid, _))| bid * (idx as u64 + 1))
        .sum()
}

pub fn process2(input: &str) -> u64 {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<u64>().expect("should be u64"),
            )
        })
        .map(|(cards, bid)| {
            (
                cards,
                bid,
                get_cards_strength(cards, true),
            )
        })
        .collect::<Vec<(&str, u64, u8)>>();
    hands.sort_by(|a, b| sort_hands(a, b, CARD_ORDER_2));
    // dbg!(&hands);
    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid, _))| bid * (idx as u64 + 1))
        .sum()
}

fn get_cards_strength(cards: &str, with_jokers: bool) -> u8 {
    let mut card_map: HashMap<char, u8> = HashMap::new();
    cards.chars().for_each(|c| {
        let count = card_map.entry(c).or_insert(0);
        *count += 1;
    });
    let values: Vec<u8> = if with_jokers {
        let jokers = *card_map.entry('J').or_default();
        card_map.remove(&'J');
        let mut values: Vec<u8> = card_map.values().cloned().collect();
        values.sort_by(|a, b| Ord::cmp(b, a));
        if !values.is_empty() {
            *values.get_mut(0).unwrap() += jokers;
        } else {
            values.push(jokers);
        }
        values
    } else {
        let mut values: Vec<u8> = card_map.values().cloned().collect();
        values.sort_by(|a, b| Ord::cmp(b, a));
        values
    };

    match (
        *values.first().unwrap(),
        *values.get(1).unwrap_or(&0),
    ) {
        (5, _) => 6,
        (4, _) => 5,
        (3, 2) => 4,
        (3, _) => 3,
        (2, 2) => 2,
        (2, _) => 1,
        _ => 0,
    }
}

fn sort_hands(
    (a_cards, _, a_strength): &(&str, u64, u8),
    (b_cards, _, b_strength): &(&str, u64, u8),
    card_order: &str,
) -> Ordering {
    let ord = Ord::cmp(&a_strength, &b_strength);
    match ord {
        Ordering::Less | Ordering::Greater => ord,
        Ordering::Equal => Iterator::zip(a_cards.chars(), b_cards.chars())
            .find_map(|(a, b)| {
                let ord = Ord::cmp(
                    &card_order.find(b).unwrap(),
                    &card_order.find(a).unwrap(),
                );
                match ord {
                    Ordering::Less | Ordering::Greater => Some(ord),
                    Ordering::Equal => None,
                }
            })
            .ok_or(Ordering::Equal)
            .unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    const EXAMPLE_1: &str = indoc! {
        "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"
    };

    const TASK: &str = include_str!("../../data/day7.task");

    #[rstest]
    #[case::n32t3k("32T3K", false, 1)]
    #[case::nt55j5("T55J5", false, 3)]
    #[case::nkk677("KK677", false, 2)]
    #[case::nktjjt("KTJJT", false, 2)]
    #[case::nqqqja("QQQJA", false, 3)]
    #[case::naaaaa("AAAAA", false, 6)]
    #[case::naa8aa("AA8AA", false, 5)]
    #[case::n23332("23332", false, 4)]
    #[case::nttt98("TTT98", false, 3)]
    #[case::n23432("23432", false, 2)]
    #[case::na23a4("A23A4", false, 1)]
    #[case::n23456("23456", false, 0)]
    #[case::jqjjq2("QJJQ2", true, 5)]
    #[case::jjkkk2("JKKK2", true, 5)]
    #[case::jqqqq2("QQQQ2", true, 5)]
    #[case::jkkja2("KKJA2", true, 3)]
    #[case::jjjjjj("JJJJJ", true, 6)]
    #[case::jjjjja("JJJJA", true, 6)]
    #[case::jjjjaa("JJJAA", true, 6)]
    #[case::j32t3k("32T3K", true, 1)]
    #[case::jt55j5("T55J5", true, 5)]
    #[case::jkk677("KK677", true, 2)]
    #[case::jktjjt("KTJJT", true, 5)]
    #[case::jqqqja("QQQJA", true, 5)]
    fn get_cards_strength_test(
        #[case] cards: &str,
        #[case] with_joker: bool,
        #[case] expected: u8,
    ) {
        let result = get_cards_strength(cards, with_joker);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example1() {
        let result = process1(EXAMPLE_1);

        assert_eq!(result, 6440);
    }

    #[test]
    fn test_task1() {
        let result = process1(TASK);

        assert_eq!(result, 248217452);
    }

    #[test]
    fn test_example2() {
        let result = process2(EXAMPLE_1);

        assert_eq!(result, 5905);
    }

    #[test]
    fn test_task2() {
        let result = process2(TASK);

        assert_eq!(result, 245576185);
    }
}
