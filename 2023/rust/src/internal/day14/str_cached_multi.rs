use std::collections::HashMap;

use super::{str_board_load::get_board_load, str_tilt_board::full_tilt_board};

pub fn process2_param_str_cached_multi(input: &str, iter_count: usize) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::c1(EXAMPLE_1, 1_000_000_000, 64)]
    fn process2_param_str_cached_multi_test(
        #[case] input: &str,
        #[case] iter_count: usize,
        #[case] expected: usize,
    ) {
        let result = process2_param_str_cached_multi(input, iter_count);

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
