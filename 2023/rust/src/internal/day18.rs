use std::{cmp, collections::VecDeque};

pub fn process1(input: &str) -> usize {
    let size_data = get_size(input);

    let hole = draw_hole(input, &size_data);
    // println!("{}", &hole);
    let reduced_hole = reduce_hole_map(&hole);
    // println!("{}", &reduced_hole);

    count_area(&reduced_hole)
}

pub fn process2(_input: &str) -> u64 {
    0
}

fn count_area(input: &str) -> usize {
    input.chars().filter(|c| c == &'#' || c == &'.').count()
}

fn reduce_hole_map(input: &str) -> String {
    let width = input.lines().next().unwrap().len();

    let mut result = input.chars().collect::<Vec<char>>();

    let mut job_queue = VecDeque::<usize>::new();
    job_queue.push_back(0);
    result[0] = ' ';

    let directions: [isize; 4] = [-(width as isize + 1), -1, 1, width as isize +1];
    while let Some(curr_idx) = job_queue.pop_front() {
        for dir in directions {
            let next_idx = curr_idx.checked_add_signed(dir);
            if let Some(next_idx) = next_idx {
                if matches!(result.get(next_idx), Some('.')) {
                    result[next_idx] = ' ';
                    job_queue.push_back(next_idx);
                }
            }
        }
    }

    result.iter().collect::<String>()
}

fn draw_hole(input: &str, size_data: &InputSizeData) -> String {
    let mut result = (0..size_data.height + 2)
        .map(|_| ".".repeat(size_data.width + 2))
        .collect::<Vec<String>>()
        .join("\n");

    let (mut x, mut y) = (
        size_data.start_x as isize + 1,
        size_data.start_y as isize + 1,
    );

    let str_width = size_data.width as isize + 1 + 2;

    input.lines().for_each(|line| {
        let mut split = line.split(' ');
        let cmd = split.next().unwrap();
        let length = split.next().unwrap().parse::<isize>().unwrap();

        match cmd {
            "U" => {
                for y in y-length..y+1 {
                    result.replace_range(
                        (y * str_width + x) as usize
                            ..(y * str_width + x + 1) as usize,
                        "#",
                    );
                }
                y -= length;
            }
            "L" => {
                result.replace_range(
                    (y * str_width + x - length) as usize
                        ..(y * str_width + x + 1) as usize,
                    &"#".repeat(length as usize + 1),
                );
                x -= length;
            }
            "D" => {
                for y in y..y + length {
                    result.replace_range(
                        (y * str_width + x) as usize
                            ..(y * str_width + x + 1) as usize,
                        "#",
                    );
                }
                y += length;
            }
            "R" => {
                result.replace_range(
                    (y * str_width + x) as usize
                        ..(y * str_width + x + length + 1) as usize,
                    &"#".repeat(length as usize + 1),
                );
                x += length;
            }
            _ => panic!("invalid cmd: {cmd}"),
        }
    });

    result
}

#[derive(Debug, PartialEq, Eq)]
struct InputSizeData {
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
}
fn get_size(input: &str) -> InputSizeData {
    let (mut minx, mut maxx, mut miny, mut maxy, mut x, mut y) = (0, 0, 0, 0, 0, 0);
    input.lines().for_each(|line| {
        let mut split = line.split(' ');
        let cmd = split.next().unwrap();
        let length = split.next().unwrap().parse::<isize>().unwrap();

        match cmd {
            "U" => {
                y -= length;
                miny = cmp::min(miny, y);
            }
            "L" => {
                x -= length;
                minx = cmp::min(minx, x);
            }
            "D" => {
                y += length;
                maxy = cmp::max(maxy, y);
            }
            "R" => {
                x += length;
                maxx = cmp::max(maxx, x);
            }
            _ => panic!("invalid cmd: {cmd}"),
        }
    });
    assert_eq!((x, y), (0, 0));
    InputSizeData {
        width: (maxx - minx + 1) as usize,
        height: (maxy - miny + 1) as usize,
        start_x: minx.unsigned_abs(),
        start_y: miny.unsigned_abs(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TASK: &str = include_str!("../../data/day18.task");
    const EXAMPLE_1: &str = indoc::indoc! {
        "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"
    };

    #[rstest]
    #[case::example(EXAMPLE_1, 62)]
    #[case::task(TASK, 40761)]
    fn process1_test(#[case] input: &str, #[case] expected: usize) {
        let result = process1(input);

        assert_eq!(result, expected);
    }
    // #[rstest]
    // #[case::c1(EXAMPLE_1, 145)]
    // fn process2_test(#[case] input: &str, #[case] expected: u64) {
    //     let result = process2(input);

    //     assert_eq!(result, expected);
    // }

    #[rstest]
    #[case::example(EXAMPLE_1, InputSizeData { width: 7, height: 10, start_x: 0, start_y: 0 })]
    #[case::task(TASK, InputSizeData { width: 373, height: 347, start_x: 0, start_y: 144 })]
    #[case::c01(indoc::indoc! {
        "R 5 (#000000)
        D 2 (#000000)
        R 1 (#000000)
        D 1 (#000000)
        L 5 (#000000)
        U 1 (#000000)
        L 1 (#000000)
        U 2 (#000000)"
    }, InputSizeData { width: 7, height: 4, start_x: 0, start_y: 0 })]
    #[case::c02(indoc::indoc! {
        "R 5 (#000000)
        U 5 (#000000)
        L 8 (#000000)
        D 5 (#000000)
        R 3 (#000000)"
    }, InputSizeData { width: 9, height: 6, start_x: 3, start_y: 5 })]
    fn get_size_test(#[case] input: &str, #[case] expected: InputSizeData) {
        let result = get_size(input);

        assert_eq!(result, expected);
    }
}
