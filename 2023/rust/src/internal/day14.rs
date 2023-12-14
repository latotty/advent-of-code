use crate::internal::col_iter::ColIter;
use rayon::prelude::*;
use std::{borrow::Cow, collections::HashMap};

pub fn process1(input: &str) -> usize {
    ColIter::new(input)
        .par_bridge()
        .map(|col| tilt_col_north(&col))
        .map(|col| get_col_load(&col.chars().collect::<Vec<char>>()))
        .sum()
}

pub fn process2(input: &str) -> usize {
    process2_param_cached_multi(input, 1_000_000_000)
}

pub fn process2_param_simple(input: &str, iter_count: usize) -> usize {
    let tilted = (0..iter_count).fold(input.to_string(), |acc, _| {
        full_tilt_board(&acc)
    });
    get_board_load(&tilted)
}

pub fn process2_param_cached(input: &str, iter_count: usize) -> usize {
    let mut cache = HashMap::<String, String>::new();
    let tilted = (0..iter_count).fold(input.to_string(), |tilted, _| {
        if let Some(cached) = cache.get(&tilted) {
            return cached.into();
        }
        let res = full_tilt_board(&tilted);
        cache.insert(tilted, res.clone());
        res
    });
    get_board_load(&tilted)
}

pub fn process2_param_cached_multi(input: &str, iter_count: usize) -> usize {
    let mut cache = HashMap::<String, (usize, String)>::new();
    let mut tilted = input.to_string();
    let mut idx = 0;
    while idx < iter_count {
        if let Some((step, cached)) = cache.get(&tilted) {
            let step = *step;
            if step + idx < iter_count {
                if let Some((step2, cached)) = cache.get(cached) {
                    let step = step + step2;
                    if step + idx < iter_count {
                        let cached = cached.clone();
                        cache
                            .entry(tilted)
                            .and_modify(|e| *e = (step, cached.clone()));
                        tilted = cached.clone();
                        idx += step;
                        continue;
                    }
                } else {
                    tilted = cached.clone();
                    idx += step;
                    continue;
                }
            }
        }
        let res = full_tilt_board(&tilted);
        cache.insert(tilted, (1, res.clone()));
        tilted = res;
        idx += 1;
    }
    get_board_load(&tilted)
}

fn tilt_col_north(input: &[char]) -> String {
    input
        .iter()
        .collect::<String>()
        .split('#')
        .map(|part| {
            if part.is_empty() {
                return part.to_string();
            }
            let mut chars = part.chars().collect::<Vec<char>>();
            chars.sort();
            chars.iter().rev().collect::<String>()
        })
        .collect::<Vec<String>>()
        .join("#")
}

fn get_col_load(input: &[char]) -> usize {
    let height = input.len();
    input
        .iter()
        .enumerate()
        .map(|(idx, c)| if c == &'O' { height - idx } else { 0 })
        .sum::<usize>()
}

fn get_board_load(input: &str) -> usize {
    ColIter::new(input).map(|col| get_col_load(&col)).sum()
}

fn full_tilt_board(input: &str) -> String {
    let a = (0..4).fold(Cow::from(input), |acc, _| {
        rotate_and_tilt_board(acc)
    });

    a.into()
}

fn rotate_and_tilt_board(board: Cow<str>) -> Cow<str> {
    ColIter::new(&board)
        .map(|col| tilt_col_north(&col).chars().rev().collect::<String>())
        .map(Cow::from)
        .reduce(|mut acc, s| {
            acc.to_mut().push('\n');
            acc.to_mut().push_str(&s);
            acc
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    const EXAMPLE_1: &str = indoc! {
        "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."
    };

    #[rstest]
    #[case::c1(EXAMPLE_1, 136)]
    fn process1_test(#[case] input: &str, #[case] expected: usize) {
        let result = process1(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(EXAMPLE_1, 64)]
    fn process2_test(#[case] input: &str, #[case] expected: usize) {
        let result = process2(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c01("OO.O.O..##", "OOOO....##")]
    #[case::c02("...OO....O", "OOO.......")]
    #[case::c03(".O...#O..O", "O....#OO..")]
    #[case::c04(".O.#......", "O..#......")]
    #[case::c05(".#.O......", ".#O.......")]
    #[case::c06("#.#..O#.##", "#.#O..#.##")]
    #[case::c07("..#...O.#.", "..#O....#.")]
    #[case::c08("....O#.O#.", "O....#O.#.")]
    #[case::c09("....#.....", "....#.....")]
    #[case::c10(".#.O.#O...", ".#O..#O...")]
    fn tilt_col_north_test(#[case] input: &str, #[case] expected: &str) {
        let result = tilt_col_north(&input.chars().collect::<Vec<char>>());

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c01("OOOO....##", 34)]
    #[case::c02("OOO.......", 27)]
    #[case::c03("O....#OO..", 17)]
    #[case::c04("O..#......", 10)]
    #[case::c05(".#O.......", 8)]
    #[case::c06("#.#O..#.##", 7)]
    #[case::c07("..#O....#.", 7)]
    #[case::c08("O....#O.#.", 14)]
    #[case::c09("....#.....", 0)]
    #[case::c10(".#O..#O...", 12)]
    fn get_col_load_test(#[case] input: &str, #[case] expected: usize) {
        let result = get_col_load(&input.chars().collect::<Vec<char>>());

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(indoc! {
        "OOOO.#.O..
        OO..#....#
        OO..O##..O
        O..#.OO...
        ........#.
        ..#....#.#
        ..O..#.O.O
        ..O.......
        #....###..
        #....#...."
    }, 136)]
    fn get_board_load_test(#[case] input: &str, #[case] expected: usize) {
        let result = get_board_load(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(indoc! {
        "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."
    }, indoc! {
        ".....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#...."
    })]
    #[case::c2(indoc! {
        ".....#....
        ....#...O#
        ...OO##...
        .OO#......
        .....OOO#.
        .O#...O#.#
        ....O#....
        ......OOOO
        #...O###..
        #..OO#...."
    }, indoc! {
        ".....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #..OO###..
        #.OOO#...O"
    })]
    #[case::c3(indoc! {
        ".....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #..OO###..
        #.OOO#...O"
    }, indoc! {
        ".....#....
        ....#...O#
        .....##...
        ..O#......
        .....OOO#.
        .O#...O#.#
        ....O#...O
        .......OOO
        #...O###.O
        #.OOO#...O"
    })]
    #[case::c3(indoc! {
        ".....#....
        ....#....#
        .....##...
        ...#......
        ........#.
        ..#....#.#
        .....#....
        ..........
        #....###..
        #....#...."
    }, indoc! {
        ".....#....
        ....#....#
        .....##...
        ...#......
        ........#.
        ..#....#.#
        .....#....
        ..........
        #....###..
        #....#...."
    })]
    fn full_tilt_board_test(#[case] input: &str, #[case] expected: &str) {
        let result = full_tilt_board(input);

        assert_eq!(result, expected);
    }
}
