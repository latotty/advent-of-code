use std::{cmp, fs, ops::Range, str::FromStr};

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

fn process2(_input: &str) -> u64 {
    // let data = input.parse::<ParsedInput>().expect("should parse");

    // let base_seed_ranges = data
    //     .seeds
    //     .chunks(2)
    //     .map(|seeds| Range {
    //         start: seeds[0],
    //         end: seeds[0] + seeds[1],
    //     })
    //     .collect::<Vec<Range<u64>>>();

    // let result_ranges: Vec<Range<u64>> =
    //     data.maps.into_iter().fold(base_seed_ranges, |acc, map| {
    //         acc.into_iter()
    //             .flat_map(|seed_range| {
    //                 let maps: Vec<(u64, u64, u64)> = map
    //                     .clone()
    //                     .into_iter()
    //                     .filter(|(_, s_s, s_r)| {
    //                         range_overlapping(
    //                             &seed_range,
    //                             &Range {
    //                                 start: *s_s,
    //                                 end: s_s + s_r,
    //                             },
    //                         )
    //                     })
    //                     .collect();
    //                 dbg!(&seed_range, &maps);
    //                 [seed_range]
    //             })
    //             .collect::<Vec<Range<u64>>>()
    //     });

    // dbg!(&result_ranges);
    0
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

    #[test]
    fn test_example1() {
        let input = EXAMPLE_1_STR;

        let result = process1(input);

        assert_eq!(result, 35);
    }

    #[ignore]
    #[test]
    fn test_example2() {
        let input = EXAMPLE_1_STR;

        let result = process2(input);

        assert_eq!(result, 46);
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
