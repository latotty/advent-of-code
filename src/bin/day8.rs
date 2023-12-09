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

fn process2(input: &str) -> u64 {
    let (instructions, map) = parse_input(input);

    let mut nodes = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<&str>>();

    let node_len = nodes.len();

    let mut step = 0;

    for instruction in instructions.iter().cycle() {
        step += 1;

        let mut end_node_count = 0;
        for node in nodes.iter_mut() {
            let (left, right) = map.get(node).unwrap();
            *node = *if *instruction { right } else { left };
            if node.ends_with('Z') {
                end_node_count += 1;
            }
        }
        if end_node_count == node_len {
            return step;
        }
    }
    panic!("should not reach");
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

    #[test]
    fn test_process1_example1() {
        let input = fs::read_to_string("./data/day8.example1").unwrap();

        let result = process1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_process1_example2() {
        let input = fs::read_to_string("./data/day8.example2").unwrap();

        let result = process1(&input);

        assert_eq!(result, 6);
    }

    #[test]
    fn test_process2_example3() {
        let input = fs::read_to_string("./data/day8.example3").unwrap();

        let result = process2(&input);

        assert_eq!(result, 6);
    }
}
