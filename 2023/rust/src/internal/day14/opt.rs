use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};

use crate::internal::col_iter::ColIter;

pub fn process2_param_opt_multi_cache(input: &str, iter_count: usize) -> usize {
    let (width, height, rocks) = parse_input(input);

    let rows = parse_rows(input);
    let cols = parse_cols(input);
    let mut last_rocks: Vec<(u8, u8)> = rocks;
    let mut new_rocks: Vec<(u8, u8)> = vec![];
    let mut cache = HashMap::<u64, (usize, Vec<(u8, u8)>)>::new();

    let mut idx = 0;
    'iter_loop: while idx < iter_count {
        let hash = get_hash(&last_rocks);
        'caching: {
            if let Some((step, cached)) = cache.get(&hash) {
                let step = *step;
                if step + idx > iter_count {
                    break 'caching;
                }
                let cached_hash = get_hash(cached);
                if let Some((step2, cached2)) = cache.get(&cached_hash) {
                    let step = step + step2;
                    if step + idx > iter_count {
                        break 'caching;
                    }
                    last_rocks = cached2.clone();
                    idx += step;
                    cache
                        .entry(hash)
                        .and_modify(|e| *e = (step, last_rocks.clone()));
                    continue 'iter_loop;
                }
                last_rocks = cached.clone();
                idx += step;
                continue 'iter_loop;
            }
        }

        FullTiltResult(last_rocks, new_rocks) = full_tilt(
            width, height, &rows, &cols, last_rocks, new_rocks,
        );

        cache.insert(hash, (1, last_rocks.clone()));
        idx += 1;
    }

    get_rocks_load(last_rocks, height)
}

pub fn process2_param_opt_shortcut(input: &str, iter_count: usize) -> usize {
    let (width, height, rocks) = parse_input(input);

    let rows = parse_rows(input);
    let cols = parse_cols(input);
    let mut last_rocks: Vec<(u8, u8)> = rocks;
    let mut new_rocks: Vec<(u8, u8)> = vec![];
    let mut cache = HashMap::<u64, usize>::new();

    let mut idx = 0;
    while idx < iter_count {
        let hash = get_hash(&last_rocks);
        if let Some(last_seen) = cache.insert(hash, idx) {
            let cycle_length = idx - last_seen;
            for _ in 0..((iter_count - idx) % cycle_length) {
                FullTiltResult(last_rocks, new_rocks) = full_tilt(
                    width, height, &rows, &cols, last_rocks, new_rocks,
                );
            }
            break;
        }

        FullTiltResult(last_rocks, new_rocks) = full_tilt(
            width, height, &rows, &cols, last_rocks, new_rocks,
        );
        idx += 1;
    }

    get_rocks_load(last_rocks, height)
}

#[inline(always)]
fn get_hash<T: Hash>(input: T) -> u64 {
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    s.finish()
}

struct FullTiltResult(Vec<(u8, u8)>, Vec<(u8, u8)>);
fn full_tilt(
    width: u8,
    height: u8,
    rows: &[Vec<u8>],
    cols: &[Vec<u8>],
    mut last_rocks: Vec<(u8, u8)>,
    mut new_rocks: Vec<(u8, u8)>,
) -> FullTiltResult {
    for tilt_dir in [Tilt::North, Tilt::West, Tilt::South, Tilt::East] {
        tilt(
            width,
            height,
            rows,
            cols,
            &last_rocks,
            &mut new_rocks,
            tilt_dir,
        );
        (last_rocks, new_rocks) = (new_rocks, last_rocks);
        new_rocks.clear();
    }

    FullTiltResult(last_rocks, new_rocks)
}

enum Tilt {
    North,
    West,
    East,
    South,
}

fn tilt(
    width: u8,
    height: u8,
    rows: &[Vec<u8>],
    cols: &[Vec<u8>],
    last_rocks: &[(u8, u8)],
    new_rocks: &mut Vec<(u8, u8)>,
    tilt: Tilt,
) {
    let (size, other_size, all_stones) = if matches!(tilt, Tilt::North | Tilt::South) {
        (width, height, cols)
    } else {
        (height, width, rows)
    };

    for coor_idx in 0..size {
        let stones = &all_stones[coor_idx as usize];
        let mut idx = 0;
        for stone in stones.iter().chain([other_size].iter()) {
            let range = idx..*stone;

            if range.is_empty() {
                idx = stone + 1;
                continue;
            }

            let rock_count = if matches!(tilt, Tilt::North | Tilt::South) {
                let mut count = 0;
                for (rx, ry) in last_rocks {
                    let ry = *ry;
                    if *rx == coor_idx && range.start <= ry && range.end > ry {
                        count += 1
                    }
                }
                count
            } else {
                let mut count = 0;
                for (rx, ry) in last_rocks {
                    let rx = *rx;
                    if *ry == coor_idx && range.start <= rx && range.end > rx {
                        count += 1
                    }
                }
                count
            };

            match tilt {
                Tilt::North => {
                    for fill_idx in idx..idx + rock_count {
                        new_rocks.push((coor_idx, fill_idx));
                    }
                }
                Tilt::West => {
                    for fill_idx in idx..idx + rock_count {
                        new_rocks.push((fill_idx, coor_idx));
                    }
                }
                Tilt::East => {
                    for fill_idx in stone - rock_count..*stone {
                        new_rocks.push((fill_idx, coor_idx));
                    }
                }
                Tilt::South => {
                    for fill_idx in stone - rock_count..*stone {
                        new_rocks.push((coor_idx, fill_idx));
                    }
                }
            }

            idx = stone + 1;
        }
    }
}

fn parse_rows(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .filter_map(|(idx, c)| if c == '#' { Some(idx as u8) } else { None })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn parse_cols(input: &str) -> Vec<Vec<u8>> {
    ColIter::new(input)
        .map(|l| {
            l.iter()
                .enumerate()
                .filter_map(|(idx, c)| if c == &'#' { Some(idx as u8) } else { None })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

type ParseInput = (u8, u8, Vec<(u8, u8)>);
fn parse_input(input: &str) -> ParseInput {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let mut rocks: Vec<(u8, u8)> = vec![];
    input
        .chars()
        .filter(|c| c != &'\n')
        .enumerate()
        .for_each(|(idx, c)| {
            if c == 'O' {
                rocks.push(((idx % width) as u8, (idx / width) as u8));
            }
        });

    (width as u8, height as u8, rocks)
}

fn get_rocks_load(rocks: Vec<(u8, u8)>, height: u8) -> usize {
    rocks.iter().map(|(_, y)| (height - y) as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::c1(EXAMPLE_1, 1, 87)]
    #[case::c100k(EXAMPLE_1, 100_000, 65)]
    #[case::c1b(EXAMPLE_1, 1_000_000_000, 64)]
    fn process2_param_opt_multi_cache_test(
        #[case] input: &str,
        #[case] iter_count: usize,
        #[case] expected: usize,
    ) {
        let result = process2_param_opt_multi_cache(input, iter_count);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(EXAMPLE_1, 1, 87)]
    #[case::c100k(EXAMPLE_1, 100_000, 65)]
    #[case::c1b(EXAMPLE_1, 1_000_000_000, 64)]
    fn process2_param_opt_shortcut_test(
        #[case] input: &str,
        #[case] iter_count: usize,
        #[case] expected: usize,
    ) {
        let result = process2_param_opt_shortcut(input, iter_count);

        assert_eq!(result, expected);
    }

    type ParseInputTest = dyn Fn((u8, u8, Vec<(u8, u8)>));

    #[rstest]
    #[case::width(EXAMPLE_1, &|(w, _, _)| assert_eq!(w, 10))]
    #[case::height(EXAMPLE_1, &|(_, h, _)| assert_eq!(h, 10))]
    #[case::rocks("..O\nO#.\n.O.", &|(_,_,r): ParseInput| assert_eq!(r, [(2, 0), (0, 1), (1, 2)]))]
    fn parse_input_test(#[case] input: &str, #[case] test: &ParseInputTest) {
        let result = parse_input(input);

        test(result);
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
