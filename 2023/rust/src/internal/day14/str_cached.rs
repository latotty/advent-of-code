use super::{str_board_load::get_board_load, str_tilt_board::full_tilt_board};
use std::collections::HashMap;

pub fn process2_param_str_cached(input: &str, iter_count: usize) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::c1(EXAMPLE_1, 100_000, 65)]
    fn process2_param_str_cached_test(
        #[case] input: &str,
        #[case] iter_count: usize,
        #[case] expected: usize,
    ) {
        let result = process2_param_str_cached(input, iter_count);

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
