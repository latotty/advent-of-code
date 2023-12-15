use crate::internal::col_iter::ColIter;

pub fn get_board_load(input: &str) -> usize {
    ColIter::new(input).map(|col| get_col_load(&col)).sum()
}

#[inline(always)]
pub fn get_col_load(input: &[char]) -> usize {
    let height = input.len();
    input
        .iter()
        .enumerate()
        .map(|(idx, c)| if c == &'O' { height - idx } else { 0 })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

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
}
