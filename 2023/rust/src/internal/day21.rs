/*
--- Day 21: Step Counter ---
You manage to catch the airship right as it's dropping someone else off on their all-expenses-paid trip to Desert Island! It even helpfully drops you off near the gardener and his massive farm.

"You got the sand flowing again! Great work! Now we just need to wait until we have enough sand to filter the water for Snow Island and we'll have snow again in no time."

While you wait, one of the Elves that works with the gardener heard how good you are at solving problems and would like your help. He needs to get his steps in for the day, and so he'd like to know which garden plots he can reach with exactly his remaining 64 steps.

He gives you an up-to-date map (your puzzle input) of his starting position (S), garden plots (.), and rocks (#). For example:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
The Elf starts at the starting position (S) which also counts as a garden plot. Then, he can take one step north, south, east, or west, but only onto tiles that are garden plots. This would allow him to reach any of the tiles marked O:

...........
.....###.#.
.###.##..#.
..#.#...#..
....#O#....
.##.OS####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
Then, he takes a second step. Since at this point he could be at either tile marked O, his second step would allow him to reach any garden plot that is one step north, south, east, or west of any tile that he could have reached after the first step:

...........
.....###.#.
.###.##..#.
..#.#O..#..
....#.#....
.##O.O####.
.##.O#...#.
.......##..
.##.#.####.
.##..##.##.
...........
After two steps, he could be at any of the tiles marked O above, including the starting position (either by going north-then-south or by going west-then-east).

A single third step leads to even more possibilities:

...........
.....###.#.
.###.##..#.
..#.#.O.#..
...O#O#....
.##.OS####.
.##O.#...#.
....O..##..
.##.#.####.
.##..##.##.
...........
He will continue like this until his steps for the day have been exhausted. After a total of 6 steps, he could reach any of the garden plots marked O:

...........
.....###.#.
.###.##.O#.
.O#O#O.O#..
O.O.#.#.O..
.##O.O####.
.##.O#O..#.
.O.O.O.##..
.##.#.####.
.##O.##.##.
...........
In this example, if the Elf's goal was to get exactly 6 more steps today, he could use them to reach any of 16 garden plots.

However, the Elf actually needs to get 64 steps today, and the map he's handed you is much larger than the example map.

Starting from the garden plot marked S on your map, how many garden plots could the Elf reach in exactly 64 steps?
*/

use std::sync::{atomic::AtomicU16, Arc};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn process1(input: &str) -> u16 {
    process1_par(input)
}

pub fn process2(_input: &str) -> u64 {
    0
}

pub fn process1_simple(input: &str) -> u16 {
    let flood_map: Vec<Option<u16>> = get_flood_map_simple(input);
    calculate_1(&flood_map, 64)
}

pub fn process1_par(input: &str) -> u16 {
    let flood_map: Vec<Option<u16>> = get_flood_map_par(input);
    calculate_1(&flood_map, 64)
}

fn calculate_1(flood_map: &[Option<u16>], exact_steps: u16) -> u16 {
    flood_map
        .iter()
        .filter(|v| {
            if let Some(v) = v {
                let v = (exact_steps as i16) - *v as i16;
                return v >= 0 && v % 2 == 0;
            }
            false
        })
        .count() as u16
}

fn _flood_map_to_string(flood_map: &[Option<u16>], width: usize) -> String {
    flood_map
        .iter()
        .map(|v| {
            if let Some(v) = v {
                (v % 10).to_string()
            } else {
                "#".to_string()
            }
        })
        .collect::<Vec<_>>()
        .chunks(width)
        .map(|v| v.join(""))
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_flood_map_simple(input: &str) -> Vec<Option<u16>> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut output = Vec::<u16>::new();
    output.resize_with(width * height, || u16::MAX);
    let input = input.chars().filter(|c| c != &'\n').collect::<Vec<_>>();

    let start_idx = input.iter().position(|c| c == &'S').unwrap();
    output[start_idx] = 0;
    let mut queue = vec![(start_idx, 0)];
    while let Some((idx, step)) = queue.pop() {
        let x = idx % width;
        let y = idx / width;
        directions.iter().for_each(|(dx, dy)| {
            if let Some(new_idx) = get_new_idx(width, height, x, y, dx, dy) {
                if input[new_idx] == '#' {
                    return;
                }
                let next_step = output[new_idx];
                if next_step > step + 1 {
                    output[new_idx] = step + 1;
                    queue.push((new_idx, step + 1));
                }
            }
        })
    }

    output
        .iter()
        .map(|v| {
            if v == &u16::MAX {
                None
            } else {
                Some(*v)
            }
        })
        .collect()
}

fn get_flood_map_par(input: &str) -> Vec<Option<u16>> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut output = Vec::<AtomicU16>::new();
    output.resize_with(width * height, || {
        AtomicU16::new(u16::MAX)
    });
    let output = Arc::new(output);
    let input = Arc::new(input.chars().filter(|c| c != &'\n').collect::<Vec<_>>());

    let start_idx = input.iter().position(|c| c == &'S').unwrap();
    output[start_idx].store(0, std::sync::atomic::Ordering::Relaxed);
    let mut queue = vec![(start_idx, 0)];
    while !queue.is_empty() {
        let new_queue = queue
            .par_iter()
            .map_with(
                (input.clone(), output.clone()),
                |(input, output), (idx, step)| {
                    let x = idx % width;
                    let y = idx / width;
                    directions
                        .iter()
                        .filter_map(|(dx, dy)| {
                            if let Some(new_idx) = get_new_idx(width, height, x, y, dx, dy) {
                                if input[new_idx] == '#' {
                                    return None;
                                }
                                let next_step =
                                    output[new_idx].load(std::sync::atomic::Ordering::Relaxed);
                                if next_step > *step + 1 {
                                    output[new_idx].store(
                                        *step + 1,
                                        std::sync::atomic::Ordering::Relaxed,
                                    );
                                    Some((new_idx, *step + 1))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                },
            )
            .flatten()
            .collect();
        queue = new_queue;
    }

    Arc::into_inner(output)
        .unwrap()
        .iter()
        .map(|v| {
            let v = v.load(std::sync::atomic::Ordering::Relaxed);
            if v == u16::MAX {
                None
            } else {
                Some(v)
            }
        })
        .collect()
}

fn get_new_idx(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    dx: &isize,
    dy: &isize,
) -> Option<usize> {
    x.checked_add_signed(*dx)
        .and_then(|x| {
            if x < width {
                y.checked_add_signed(*dy).map(|y| {
                    if y < height {
                        Some(x + y * height)
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
        .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::rstest;

    const EXAMPLE_1: &str = indoc! {
        "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ..........."
    };

    #[rstest]
    #[case::c01(indoc! {
        "...
        .S.
        ..."
    }, indoc! {
        "212
        101
        212"
    })]
    #[case::c02(indoc! {
        ".#.
        .S#
        ..."
    }, indoc! {
        "2##
        10#
        212"
    })]
    fn get_flood_map_simple_test(#[case] input: &str, #[case] expected: &str) {
        let expected = expected
            .lines()
            .flat_map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).map(|n| n as u16))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let result = get_flood_map_simple(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c01(indoc! {
        "...
        .S.
        ..."
    }, indoc! {
        "212
        101
        212"
    })]
    #[case::c02(indoc! {
        ".#.
        .S#
        ..."
    }, indoc! {
        "2##
        10#
        212"
    })]
    fn get_flood_map_par_test(#[case] input: &str, #[case] expected: &str) {
        let expected = expected
            .lines()
            .flat_map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).map(|n| n as u16))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let result = get_flood_map_par(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::example_1(EXAMPLE_1, 1, 2)]
    #[case::example_2(EXAMPLE_1, 2, 4)]
    #[case::example_3(EXAMPLE_1, 3, 6)]
    #[case::example_6(EXAMPLE_1, 6, 16)]
    fn calculate_1_test(#[case] input: &str, #[case] steps: u16, #[case] expected: u16) {
        let flood_map: Vec<Option<u16>> = get_flood_map_par(input);
        let result = calculate_1(&flood_map, steps);

        assert_eq!(result, expected);
    }

    // #[rstest]
    // #[case::c1(EXAMPLE_1, 16)]
    // fn process1_test(#[case] input: &str, #[case] expected: u32) {
    //     let result = process1(input);

    //     assert_eq!(result, expected);
    // }

    // #[rstest]
    // #[case::example(EXAMPLE_1, 167409079868000)]
    // fn process2_test(#[case] input: &str, #[case] expected: u64) {
    //     let result = process2(input);

    //     assert_eq!(result, expected);
    // }
}
