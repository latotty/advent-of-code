use super::col_iter::ColIter;

pub fn process1(input: &str) -> usize {
    let (xempty, yempty) = get_empty_space(input);

    let stars = get_stars(input);

    calculate_start_distance(stars, xempty, yempty, 2)
}

fn calculate_start_distance(
    stars: Vec<(usize, usize)>,
    xempty: Vec<usize>,
    yempty: Vec<usize>,
    empty_space_size: usize,
) -> usize {
    let mut res = 0;
    for (i, star1) in stars.iter().enumerate() {
        for star2 in stars[(i + 1)..].iter() {
            let minx = star1.0.min(star2.0);
            let maxx = star1.0.max(star2.0);
            let miny = star1.1.min(star2.1);
            let maxy = star1.1.max(star2.1);
            let xdiff = maxx - minx
                + xempty.iter().filter(|x| x < &&maxx && x > &&minx).count()
                    * (empty_space_size - 1);
            let ydiff = maxy - miny
                + yempty.iter().filter(|y| y < &&maxy && y > &&miny).count()
                    * (empty_space_size - 1);
            res += xdiff + ydiff;
        }
    }

    res
}

fn get_stars(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect()
}

fn get_empty_space(input: &str) -> (Vec<usize>, Vec<usize>) {
    let yempty = input
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| if line.contains('#') { None } else { Some(idx) })
        .collect::<Vec<usize>>();

    let xempty = ColIter::new(input)
        .enumerate()
        .filter_map(|(idx, chars)| {
            if chars.contains(&'#') {
                None
            } else {
                Some(idx)
            }
        })
        .collect::<Vec<usize>>();

    (xempty, yempty)
}

pub fn process2(input: &str) -> usize {
    let (xempty, yempty) = get_empty_space(input);

    let stars = get_stars(input);

    calculate_start_distance(stars, xempty, yempty, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    #[rstest]
    #[case::c1(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, 374)]
    fn test_process1_example(#[case] input: &str, #[case] expected: usize) {
        let result = process1(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, (vec![2, 5, 8], vec![3, 7]))]
    fn get_empty_space_test(#[case] input: &str, #[case] expected: (Vec<usize>, Vec<usize>)) {
        let result = get_empty_space(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, vec![(0, 2), (0, 9), (1, 5), (3, 0), (4, 9), (6, 4), (7, 1), (7, 8), (9, 6)])]
    fn get_stars_test(#[case] input: &str, #[case] expected: Vec<(usize, usize)>) {
        let mut result = get_stars(input);
        let mut expected = expected.clone();
        result.sort();
        expected.sort();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, 2, 374)]
    #[case::c1(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, 10, 1030)]
    #[case::c1(indoc! {"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    "}, 100, 8410)]
    fn calculate_start_distance_test(
        #[case] input: &str,
        #[case] empty_space_size: usize,
        #[case] expected: usize,
    ) {
        let (xempty, yempty) = get_empty_space(input);
        let stars = get_stars(input);
        let result = calculate_start_distance(stars, xempty, yempty, empty_space_size);

        assert_eq!(result, expected);
    }

    // #[ignore = "not yet"]
    // #[test]
    // fn test_process1_task() {
    //     let input = fs::read_to_string("./data/day11.task").unwrap();

    //     let result = process1(&input);

    //     assert_eq!(result, 7030);
    // }

    // #[rstest]
    // #[case::c1(&[
    //     "...........",
    //     ".S-------7.",
    //     ".|F-----7|.",
    //     ".||.....||.",
    //     ".||.....||.",
    //     ".|L-7.F-J|.",
    //     ".|..|.|..|.",
    //     ".L--J.L--J.",
    //     "...........",
    // ].join("\n"), 4)]
    // fn test_process2_example(#[case] input: &str, #[case] expected: usize) {
    //     let result = process2(input);

    //     assert_eq!(result, expected);
    // }
}
