use super::str_simple::process1_str_simple;

pub fn process1(input: &str) -> usize {
    process1_str_simple(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::c1(EXAMPLE_1, 136)]
    fn process1_test(#[case] input: &str, #[case] expected: usize) {
        let result = process1(input);

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
