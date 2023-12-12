use rayon::prelude::*;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./data/day12.task").unwrap();

    let result1 = process1(&input);

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}

fn process1(input: &str) -> usize {
    input.lines().par_bridge().map(get_line_combinations).sum()
}

fn process2(input: &str) -> usize {
    input
        .lines()
        .par_bridge()
        .map(|line| get_line_combinations(&repeat_input(line, 5)))
        .sum()
}

fn get_line_combinations(input: &str) -> usize {
    let mut parts = input.split(' ');
    let springs = parts.next().unwrap();
    let cgroups = parts
        .next()
        .unwrap()
        .split(',')
        .map(|s| str::parse::<usize>(s).unwrap())
        .collect::<Vec<usize>>();

    let mut cache = HashMap::new();

    fn rec(
        cache: &mut HashMap<String, usize>,
        springs: Option<&str>,
        cgroups: Option<&[usize]>,
    ) -> usize {
        let (cgroups, springs) = match (cgroups, springs) {
            (None | Some([]), _) => return 1,
            (Some(cgroups), Some(springs)) if !cgroups.is_empty() && !springs.is_empty() => {
                (cgroups, springs)
            }
            _ => return 0,
        };
        if springs.len() < cgroups.iter().sum::<usize>() + cgroups.len() - 1 {
            return 0;
        }
        let key = format!("{} {:?}", springs, cgroups);
        if let Some(res) = cache.get(&key) {
            return *res;
        }
        let first_cgroup = *cgroups.first().unwrap();
        let found = !springs[0..first_cgroup].contains('.')
            && !matches!(
                springs.get(first_cgroup..first_cgroup + 1),
                Some("#")
            );
        let starts_with_hash = springs[0..1] == *"#";
        let rest_is_ok = cgroups.len() > 1
            || !match springs.get(first_cgroup + 1..) {
                Some(rest) => rest.contains('#'),
                None => false,
            };

        let res = match (found, rest_is_ok, starts_with_hash) {
            (true, true, false) => {
                rec(
                    cache,
                    springs.get(first_cgroup + 1..),
                    cgroups.get(1..),
                ) + rec(cache, springs.get(1..), Some(cgroups))
            }
            (true, true, true) => rec(
                cache,
                springs.get(first_cgroup + 1..),
                cgroups.get(1..),
            ),
            (_, _, true) => 0,
            (_, _, false) => rec(cache, springs.get(1..), Some(cgroups)),
        };
        cache.insert(key, res);
        res
    }

    rec(
        &mut cache,
        Some(springs),
        Some(&cgroups),
    )
}

fn repeat_input(input: &str, repeat: usize) -> String {
    let mut parts = input.split(' ');
    let springs = parts.next().unwrap();
    let cgroups = parts.next().unwrap();

    format!(
        "{} {}",
        [springs].repeat(repeat).join("?"),
        [cgroups].repeat(repeat).join(",")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::c1("???.### 1,1,3", 1)]
    #[case::c2(".??..??...?##. 1,1,3", 4)]
    #[case::c3("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case::c4("????.#...#... 4,1,1", 1)]
    #[case::c5("????.######..#####. 1,6,5", 4)]
    #[case::c6("?###???????? 3,2,1", 10)]
    #[case::c7("??#...?#???? 2,6", 1)]
    #[case::c8("??#??.???#?. 1,3,2", 2)]
    #[case::c9("??..???### 1,4", 3)]
    #[case::c10("???????????#??? 5,2,1", 22)]
    #[case::c11(".##.?#??.#.?# 2,1,1,1", 1)]
    #[case::c12("### 1", 0)]
    #[case::c13("### 2", 0)]
    #[case::c14("### 3", 1)]
    #[case::c15("### 4", 0)]
    fn get_line_combinations_test(#[case] input: &str, #[case] expected: usize) {
        let result = get_line_combinations(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1("???.### 1,1,3", 2, "???.###????.### 1,1,3,1,1,3")]
    #[case::c1(
        "???.### 1,1,3",
        5,
        "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
    )]
    fn repeat_input_test(#[case] input: &str, #[case] repeat: usize, #[case] expected: &str) {
        let result = repeat_input(input, repeat);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1("???.### 1,1,3", 1)]
    #[case::c2(".??..??...?##. 1,1,3", 16384)]
    #[case::c3("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case::c4("????.#...#... 4,1,1", 16)]
    #[case::c5("????.######..#####. 1,6,5", 2500)]
    #[case::c6("?###???????? 3,2,1", 506250)]
    #[case::c7("### 1", 0)]
    #[case::c8("### 2", 0)]
    #[case::c9("### 3", 1)]
    #[case::c10("### 4", 0)]
    #[case::c11(".# 1", 1)]
    fn get_line_combinations_repeat_test(#[case] input: &str, #[case] expected: usize) {
        let result = get_line_combinations(repeat_input(input, 5).as_str());

        assert_eq!(result, expected);
    }
}
