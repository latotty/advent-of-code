use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./data/day8.task").unwrap();

    let result1 = process1(&input);

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}

fn process1(input: &str) -> u64 {
    const START_NODE: &str = "AAA";
    const END_NODE: &str = "ZZZ";

    let (instructions, map) = parse_input(input);

    let mut node = START_NODE;
    let mut step = 0;
    for instruction in instructions.iter().cycle() {
        step += 1;
        let (left, right) = map.get(node).unwrap();
        node = *if *instruction { right } else { left };
        if node == END_NODE {
            return step;
        }
    }
    panic!("should not reach");
}

fn process2(input: &str) -> usize {
    let (instructions, map) = parse_input(input);

    let mut nodes = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .map(Some)
        .collect::<Vec<Option<&str>>>();

    let mut last: Vec<Option<usize>> = vec![None; nodes.len()];
    let mut freq: Vec<Option<usize>> = vec![None; nodes.len()];
    for (instr_idx, instruction) in instructions.iter().cycle().enumerate() {
        for (node_idx, node) in nodes.iter_mut().enumerate() {
            if let Some(node_val) = node {
                let (left, right) = map.get(node_val).unwrap();
                let next = *if *instruction { right } else { left };
                *node = Some(next);
                if next.ends_with('Z') {
                    if let Some(last) = last[node_idx] {
                        freq[node_idx] = Some(instr_idx - last);
                        *node = None;
                    } else {
                        last[node_idx] = Some(instr_idx);
                    }

                    if freq.iter().all(|e| e.is_some()) {
                        return lcmm(&freq.iter().flatten().cloned().collect::<Vec<usize>>());
                    }
                }
            }
        }
    }
    panic!("should not reach");
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcmm(nums: &[usize]) -> usize {
    let mut iter = nums.iter();
    let first = *(iter.next().unwrap());
    nums.iter().fold(first, |acc, num| lcm(acc, *num))
}

fn lcm(a: usize, b: usize) -> usize {
    let gcd = gcd(a, b);
    a * b / gcd
}

fn parse_input(input: &str) -> (Vec<bool>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines();

    let instructions = lines
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|c| c == 'R')
        .collect::<Vec<bool>>();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in lines.skip(1) {
        let mut line = line.split(" = ");
        let key = line.next().unwrap();
        let mut value = line
            .next()
            .unwrap()
            .trim_matches(|c| char::is_ascii_punctuation(&c))
            .split(", ");
        map.insert(
            key,
            (
                value.next().unwrap(),
                value.next().unwrap(),
            ),
        );
    }

    (instructions, map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {
        "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"
    };

    const EXAMPLE_2: &str = indoc! {
        "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
    };

    const EXAMPLE_3: &str = indoc! {
        "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"
    };

    #[test]
    fn test_process1_example1() {
        let result = process1(EXAMPLE_1);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_process1_example2() {
        let result = process1(EXAMPLE_2);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_process2_example3() {
        let result = process2(EXAMPLE_3);

        assert_eq!(result, 6);
    }

    #[rstest]
    #[case::c1(12, 16, 4)]
    #[case::c2(19783, 16531, 271)]
    fn gcd_test(#[case] a: usize, #[case] b: usize, #[case] expected: usize) {
        let result = gcd(a, b);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(12, 16, 48)]
    #[case::c2(19783, 16531, 1206763)]
    fn lcm_test(#[case] a: usize, #[case] b: usize, #[case] expected: usize) {
        let result = lcm(a, b);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(&vec![12, 16], 48)]
    #[case::c2(&vec![19783, 16531], 1206763)]
    #[case::c3(&vec![100, 23, 98], 112700)]
    fn lcmm_test(#[case] a: &[usize], #[case] expected: usize) {
        let result = lcmm(a);

        assert_eq!(result, expected);
    }
}
