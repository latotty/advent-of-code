use std::{cmp, ops::ControlFlow};

use rayon::prelude::*;

use crate::internal::col_iter::ColIter;

pub fn process1(input: &str) -> u64 {
    input
        .split("\n\n")
        .par_bridge()
        .map(get_block_mirror_num)
        .sum()
}

pub fn process2(input: &str) -> u64 {
    input
        .split("\n\n")
        .par_bridge()
        .map(get_block_mirror_smudge_num)
        .sum()
}

fn get_block_mirror_num(input: &str) -> u64 {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let x_mirror = (1..width).find(|x| {
        input.lines().all(|line| {
            let left = &line[0..*x];
            let right = &line[*x..];
            let min_slice_width = cmp::min(left.len(), right.len());
            left.chars()
                .rev()
                .take(min_slice_width)
                .eq(right.chars().take(min_slice_width))
        })
    });

    if let Some(x_mirror) = x_mirror {
        return x_mirror as u64;
    }

    let y_mirror = (1..height).find(|y| {
        ColIter::new(input).all(|col| {
            let left = &col[0..*y];
            let right = &col[*y..];
            let min_slice_width = cmp::min(left.len(), right.len());
            left.iter()
                .rev()
                .take(min_slice_width)
                .eq(right.iter().take(min_slice_width))
        })
    });

    if let Some(y_mirror) = y_mirror {
        return y_mirror as u64 * 100;
    }

    panic!("should have mirror {input}");
}

fn get_block_mirror_smudge_num(input: &str) -> u64 {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let x_mirror = (1..width).find(|x| {
        matches!(
            input.lines().try_fold(0, |smudges, line| {
                let left = &line[0..*x];
                let right = &line[*x..];
                let min_slice_width = cmp::min(left.len(), right.len());

                let new_smudges = left
                    .chars()
                    .rev()
                    .take(min_slice_width)
                    .zip(right.chars().take(min_slice_width))
                    .filter(|(a, b)| a != b)
                    .count();
                let smudges = smudges + new_smudges;

                if smudges > 1 {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(smudges)
                }
            }),
            ControlFlow::Continue(1)
        )
    });

    if let Some(x_mirror) = x_mirror {
        return x_mirror as u64;
    }

    let y_mirror = (1..height).find(|y| {
        matches!(
            ColIter::new(input).try_fold(0, |smudges, col| {
                let left = &col[0..*y];
                let right = &col[*y..];
                let min_slice_width = cmp::min(left.len(), right.len());

                let new_smudges = left
                    .iter()
                    .rev()
                    .take(min_slice_width)
                    .zip(right.iter().take(min_slice_width))
                    .filter(|(a, b)| a != b)
                    .count();
                let smudges = smudges + new_smudges;

                if smudges > 1 {
                    ControlFlow::Break(())
                } else {
                    ControlFlow::Continue(smudges)
                }
            }),
            ControlFlow::Continue(1)
        )
    });

    if let Some(y_mirror) = y_mirror {
        return y_mirror as u64 * 100;
    }

    panic!("should have mirror {input}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    const EXAMPLE_1: &str = indoc! {
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
    };

    #[rstest]
    #[case::c1(EXAMPLE_1, 405)]
    fn process1_test(#[case] input: &str, #[case] expected: u64) {
        let result = process1(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(EXAMPLE_1, 400)]
    fn process2_test(#[case] input: &str, #[case] expected: u64) {
        let result = process2(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(indoc! {
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#."
    }, 5)]
    #[case::c2(indoc! {
        "#...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
    }, 400)]
    #[case::c3(indoc! {
        "##..
        ..##
        ..#."
    }, 1)]
    #[case::c4(indoc! {
        "##...
        ..###
        ..##."
    }, 1)]
    #[case::c5(indoc! {
        "#..#
        .##."
    }, 2)]
    #[case::c6(indoc! {
        "##
        ..
        ##"
    }, 1)]
    #[case::c7(indoc! {
        "#..#
        .##.
        #..#"
    }, 2)]
    #[case::c7(indoc! {
        ".....######......
        ####.##..##.####.
        ##..#.####.#..##.
        .#..#.....##..#.#
        .#..##.##.##..#..
        .#...######...#..
        #.#..........#.##
        .###.#.##.#.###..
        .###.#.##.#.###.."
    }, 800)]
    fn get_block_mirror_num_test(#[case] input: &str, #[case] expected: u64) {
        let result = get_block_mirror_num(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(indoc! {
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#."
    }, 300)]
    #[case::c2(indoc! {
        "#...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
    }, 100)]
    fn get_block_mirror_smudge_num_test(#[case] input: &str, #[case] expected: u64) {
        let result = get_block_mirror_smudge_num(input);

        assert_eq!(result, expected);
    }

    /*
     21021 too low
    */
}
