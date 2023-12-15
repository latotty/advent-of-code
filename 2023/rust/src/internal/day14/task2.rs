use super::opt::process2_param_opt;

pub fn process2(input: &str) -> usize {
    process2_param_opt(input, 1_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::c1(EXAMPLE_1, 64)]
    fn process2_test(#[case] input: &str, #[case] expected: usize) {
        let result = process2(input);

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
