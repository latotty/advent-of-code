use std::{
    cmp::{self},
    fs,
    ops::Range,
    str::FromStr,
};

fn main() {
    let input = fs::read_to_string("./data/day5.task").unwrap();

    let result1 = process1(&input.clone());

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}

fn process1(input: &str) -> u64 {
    let data = input.parse::<ParsedInput>().expect("should parse");

    data.seeds
        .into_iter()
        .map(|seed| {
            data.maps.clone().into_iter().fold(seed, |last, map| {
                for (d_start, s_start, range) in map {
                    if last >= s_start && last < s_start + range {
                        return d_start + last - s_start;
                    }
                }
                last
            })
        })
        .min()
        .expect("should have min")
}

fn process2(input: &str) -> u64 {
    let data = input.parse::<ParsedInput>().expect("should parse");

    let base_seed_ranges = data
        .seeds
        .chunks(2)
        .map(|seeds| Range {
            start: seeds[0],
            end: seeds[0] + seeds[1],
        })
        .collect::<Vec<Range<u64>>>();

    let result_ranges: Vec<Range<u64>> =
        data.maps.into_iter().fold(base_seed_ranges, |acc, map| {
            let next = acc
                .into_iter()
                .flat_map(|seed_range| {
                    let mappers: Vec<(u64, u64, u64)> = map
                        .clone()
                        .into_iter()
                        .filter(|(_, s_s, s_r)| {
                            range_overlapping(
                                &seed_range,
                                &Range {
                                    start: *s_s,
                                    end: s_s + s_r,
                                },
                            )
                        })
                        .collect();

                    mappers
                        .iter()
                        .fold(
                            vec![seed_range.clone()],
                            |acc, (_, start, range)| {
                                acc.iter()
                                    .flat_map(|seed_range| {
                                        split_range_by_range(
                                            seed_range,
                                            &Range {
                                                start: *start,
                                                end: start + range,
                                            },
                                        )
                                    })
                                    .collect()
                            },
                        )
                        .iter()
                        .map(|seed_range| {
                            if let Some((d_s, s_s, _)) = mappers.iter().find(|(_, s_s, s_r)| {
                                range_overlapping(
                                    seed_range,
                                    &Range {
                                        start: *s_s,
                                        end: s_s + s_r,
                                    },
                                )
                            }) {
                                return Range {
                                    start: seed_range.start + d_s - s_s,
                                    end: seed_range.end + d_s - s_s,
                                };
                            };
                            seed_range.clone()
                        })
                        .collect::<Vec<Range<u64>>>()
                })
                .collect::<Vec<Range<u64>>>();

            // dbg!(&next);

            merge_ranges(next)
        });

    dbg!(&result_ranges);
    result_ranges.first().expect("should have first").start
}

fn split_range_by_range(a: &Range<u64>, b: &Range<u64>) -> Vec<Range<u64>> {
    match (a.contains(&b.start), a.contains(&b.end)) {
        (true, true) => vec![
            Range {
                start: a.start,
                end: b.start,
            },
            Range {
                start: b.start,
                end: b.end,
            },
            Range {
                start: b.end,
                end: a.end,
            },
        ],
        (true, false) => vec![
            Range {
                start: a.start,
                end: b.start,
            },
            Range {
                start: b.start,
                end: a.end,
            },
        ],
        (false, true) => vec![
            Range {
                start: a.start,
                end: b.end,
            },
            Range {
                start: b.end,
                end: a.end,
            },
        ],
        (false, false) => vec![a.clone()],
    }
}

fn merge_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_by(|a, b| Ord::cmp(&a.start, &b.start));

    let mut result: Vec<Range<u64>> = vec![];
    let mut prev: Option<Range<u64>> = None;
    for r in &ranges {
        if let Some(prev_some) = prev {
            if range_overlapping(&prev_some, r) || prev_some.end == r.start {
                let merged = Range {
                    start: cmp::min(prev_some.start, r.start),
                    end: cmp::max(prev_some.end, r.end),
                };
                prev = Some(merged);
                continue;
            }
            result.push(prev_some);
        }
        prev = Some(r.clone());
    }
    if let Some(prev) = prev {
        result.push(prev);
    }
    result
}

fn range_overlapping(r1: &Range<u64>, r2: &Range<u64>) -> bool {
    cmp::max(r1.start, r2.start) < cmp::min(r1.end, r2.end)
}

#[derive(Debug, PartialEq)]
struct ParsedInput {
    pub seeds: Vec<u64>,
    pub maps: Vec<Vec<(u64, u64, u64)>>,
}

impl FromStr for ParsedInput {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let seeds = input
            .lines()
            .take(1)
            .flat_map(|l| l.split(": "))
            .skip(1)
            .flat_map(|l| l.split(' '))
            .map(|s| s.parse::<u64>().expect("should be num"))
            .collect::<Vec<u64>>();

        let maps = input
            .split("\n\n")
            .skip(1)
            .map(|map_block| {
                map_block
                    .split('\n')
                    .skip(1)
                    .map(|line| {
                        let mut nums = line
                            .split(' ')
                            .map(|n| n.parse::<u64>().expect("should be num"));
                        (
                            nums.next().expect("should have 3 tuple"),
                            nums.next().expect("should have 3 tuple"),
                            nums.next().expect("should have 3 tuple"),
                        )
                    })
                    .collect::<Vec<(u64, u64, u64)>>()
            })
            .collect::<Vec<Vec<(u64, u64, u64)>>>();

        Ok(ParsedInput { seeds, maps })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(EXAMPLE_1_STR, ParsedInput {
        seeds: vec![79, 14, 55, 13],
        maps: vec![
            vec![(50, 98, 2), (52, 50, 48)],
            vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)],
            vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)],
            vec![(88, 18, 7), (18, 25, 70)],
            vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)],
            vec![(0, 69, 1), (1, 0, 69)],
            vec![(60, 56, 37), (56, 93, 4)],
        ],
    })]
    fn input_parse_test(#[case] input: &str, #[case] expected: ParsedInput) {
        let result = input.parse::<ParsedInput>().unwrap();

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0..1, 0..1, true)]
    #[case(0..10, 2..3, true)]
    #[case(0..1, 1..2, false)]
    #[case(0..0, 0..0, false)]
    fn range_overlapping_test(
        #[case] a: Range<u64>,
        #[case] b: Range<u64>,
        #[case] expected: bool,
    ) {
        let result = range_overlapping(&a, &b);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(vec![], vec![])]
    #[case(vec![0..1, 1..2], vec![0..2])]
    #[case(vec![0..1, 2..3], vec![0..1, 2..3])]
    #[case(vec![0..10, 2..3], vec![0..10])]
    #[case(vec![0..10, 2..10], vec![0..10])]
    #[case(vec![0..10, 0..1], vec![0..10])]
    #[case(vec![2..10, 0..2], vec![0..10])]
    fn merge_ranges_test(#[case] input: Vec<Range<u64>>, #[case] expected: Vec<Range<u64>>) {
        let result = merge_ranges(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0..1, 1..2, vec![0..1])]
    #[case(0..10, 1..2, vec![0..1, 1..2, 2..10])]
    #[case(0..10, 8..11, vec![0..8, 8..10])]
    #[case(1..10, 0..3, vec![1..3, 3..10])]
    fn split_range_by_range_test(
        #[case] a: Range<u64>,
        #[case] b: Range<u64>,
        #[case] expected: Vec<Range<u64>>,
    ) {
        let result = split_range_by_range(&a, &b);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_example1() {
        let input = EXAMPLE_1_STR;

        let result = process1(input);

        assert_eq!(result, 35);
    }

    #[test]
    fn test_example2() {
        let input = EXAMPLE_1_STR;

        let result = process2(input);

        assert_eq!(result, 46);
    }

    const EXAMPLE_1_STR: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}
