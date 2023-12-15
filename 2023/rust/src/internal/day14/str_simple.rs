use super::{
    str_board_load::{get_board_load, get_col_load},
    str_tilt_board::{full_tilt_board, tilt_col_north},
};
use crate::internal::col_iter::ColIter;
use rayon::prelude::*;

pub fn process1_str_simple(input: &str) -> usize {
    ColIter::new(input)
        .par_bridge()
        .map(|col| tilt_col_north(&col))
        .map(|col| get_col_load(&col.chars().collect::<Vec<char>>()))
        .sum()
}

pub fn process2_param_str_simple(input: &str, iter_count: usize) -> usize {
    let tilted = (0..iter_count).fold(input.to_string(), |acc, _| {
        full_tilt_board(&acc)
    });
    get_board_load(&tilted)
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::c1(EXAMPLE_1, 136)]
    fn process1_test(#[case] input: &str, #[case] expected: usize) {
        let result = process1_str_simple(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(EXAMPLE_1, 1, 87)]
    #[case::c1k(EXAMPLE_1, 1_000, 64)]
    fn process2_param_str_simple_test(
        #[case] input: &str,
        #[case] iter_count: usize,
        #[case] expected: usize,
    ) {
        let result = process2_param_str_simple(input, iter_count);

        assert_eq!(result, expected);
    }

    pub const EXAMPLE_1: &str = indoc::indoc! {
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
}
