use crate::internal::col_iter::ColIter;
use std::borrow::Cow;

pub fn full_tilt_board(input: &str) -> String {
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
pub fn tilt_col_north(input: &[char]) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

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
