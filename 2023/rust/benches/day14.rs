use aoc2023::internal::day14::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = include_str!("../data/day14.task");
    let result1 = process1(input);
    assert_eq!(result1, 113486);
}

#[divan::bench]
fn part2() {
    let input = include_str!("../data/day14.task");
    let result1 = process2(input);
    assert_eq!(result1, 104409);
}

const EXAMPLE_1: &str = indoc::indoc! {
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

// FROM https://www.reddit.com/r/adventofcode/comments/18i45eo/2023_day_14_part_2_worst_case_complexity/
const CYCLE_2520: &str = indoc::indoc! {
    "......................
    .................#....
    ......#...............
    .........#............
    ........#O.#..........
    ..........#..#........
    .......#....#..#......
    .........#....#.......
    ...........#....#.....
    .............#....#...
    ...............#......
    ......................
    ...................#..
    ......#...............
    .........#............
    ........#O.#..........
    ..........#..#........
    .......#....#..#......
    .........#....#..#....
    ...........#....#.....
    .............#....#...
    ...............#....#.
    .................#....
    .....................#
    ......#...............
    .........#............
    ........#O.#..........
    ..........#..#........
    .......#....#..#......
    .........#....#..#....
    ...........#....#..#..
    .............#....#...
    ...............#....#.
    .................#....
    ...................#..
    .....................#
    ...#..................
    ......#...............
    .....#O.#.............
    .......#..#...........
    ....#....#..#.........
    ......#....#..#.......
    ........#....#..#.....
    ..........#....#..#...
    ............#....#....
    ..............#....#..
    ................#.....
    ..................#...
    .....................#
    .#....................
    ....#.................
    ...#O.#...............
    .....#..#.............
    ..#....#..#...........
    ....#....#..#.........
    ......#....#..#.......
    ........#....#..#.....
    ..........#....#..#...
    ............#....#....
    ..............#....#..
    ................#.....
    ..................#...
    ......................"
};

macro_rules! part2_bench_part {
    (
        $name:ident,
        $fn:ident,
        $input:expr,
        $iter_count:expr,
        $result:expr
    ) => {
        #[divan::bench(
                    max_time = std::time::Duration::from_millis(200)
                )]
        fn $name() {
            let result1 = $fn(EXAMPLE_1, $iter_count);
            assert_eq!(result1, $result);
        }
    };
}

macro_rules! part2_ex_bench {
    (
        $name:ident,
        $fn:ident,
        $input:expr,
        10k
    ) => {
        mod $name {
            use super::*;

            part2_bench_part!(it_10, $fn, $input, 10, 69);
            part2_bench_part!(it_100, $fn, $input, 100, 68);
            part2_bench_part!(it_1000, $fn, $input, 1_000, 64);
            part2_bench_part!(it_10000, $fn, $input, 10_000, 69);
        }
    };
    (
        $name:ident,
        $fn:ident,
        $input:expr,
        1m
    ) => {
        mod $name {
            use super::*;

            part2_bench_part!(it_10, $fn, $input, 10, 69);
            part2_bench_part!(it_100, $fn, $input, 100, 68);
            part2_bench_part!(it_1000, $fn, $input, 1_000, 64);
            part2_bench_part!(it_10000, $fn, $input, 10_000, 69);
            part2_bench_part!(it_100000, $fn, $input, 100_000, 65);
            part2_bench_part!(it_1000000, $fn, $input, 1_000_000, 63);
        }
    };
    (
        $name:ident,
        $fn:ident,
        $input:expr,
        1M
    ) => {
        mod $name {
            use super::*;

            part2_bench_part!(it_10, $fn, $input, 10, 69);
            part2_bench_part!(it_100, $fn, $input, 100, 68);
            part2_bench_part!(it_1000, $fn, $input, 1_000, 64);
            part2_bench_part!(it_10000, $fn, $input, 10_000, 69);
            part2_bench_part!(it_100000, $fn, $input, 100_000, 65);
            part2_bench_part!(it_1000000, $fn, $input, 1_000_000, 63);
            part2_bench_part!(it_10000000, $fn, $input, 10_000_000, 69);
            part2_bench_part!(
                it_100000000,
                $fn,
                $input,
                100_000_000,
                68
            );
            part2_bench_part!(
                it_1000000000,
                $fn,
                $input,
                1_000_000_000,
                64
            );
        }
    };
}

part2_ex_bench!(
    process2_param_simple_ex,
    process2_param_simple,
    EXAMPLE_1,
    10k
);
part2_ex_bench!(
    process2_param_cached_ex,
    process2_param_cached,
    EXAMPLE_1,
    1m
);
part2_ex_bench!(
    process2_param_cached_multi_ex,
    process2_param_cached_multi,
    EXAMPLE_1,
    1M
);

part2_ex_bench!(
    process2_param_simple_c2520,
    process2_param_simple,
    CYCLE_2520,
    10k
);
part2_ex_bench!(
    process2_param_cached_c2520,
    process2_param_cached,
    CYCLE_2520,
    1m
);
part2_ex_bench!(
    process2_param_cached_multi_c2520,
    process2_param_cached_multi,
    CYCLE_2520,
    1M
);
