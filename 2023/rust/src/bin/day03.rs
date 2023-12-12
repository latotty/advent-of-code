use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fs;
use std::ops::Range;

fn main() {
    let input = fs::read_to_string("./data/day3.task").unwrap();

    let result1 = process1(&input);

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

fn process1(input: &str) -> usize {
    let mut result = 0;

    let lines: Vec<&str> = input.lines().collect();

    for (line_num, line) in input.lines().enumerate() {
        for cap in NUM_RE.captures_iter(line) {
            let num_match: Option<regex::Match<'_>> = cap.get(0);
            if let Some(num_match) = num_match {
                let range = num_match.range();
                let range_box: [usize; 4] = [
                    line_num.saturating_sub(1),
                    range.start.saturating_sub(1),
                    line_num + 1,
                    range.end + 1,
                ];

                for line in lines[range_box[0]..cmp::min(range_box[2], lines.len() - 1) + 1].iter()
                {
                    let line = &line[range_box[1]..cmp::min(range_box[3], line.len() - 1)];
                    if line
                        .find(|c: char| !c.is_ascii_alphanumeric() && c != '.')
                        .is_some()
                    {
                        result += num_match.as_str().parse::<usize>().unwrap();
                        break;
                    }
                }
            }
        }
    }

    result
}

fn process2(input: &str) -> usize {
    let mut result = 0;

    let lines: Vec<&str> = input.lines().collect();

    for (line_num, line) in lines.clone().into_iter().enumerate() {
        for (match_index, _) in line.match_indices('*') {
            let match_range = Range {
                start: match_index.saturating_sub(1),
                end: match_index + 2,
            };
            let gear_nums = lines[line_num.saturating_sub(1)..line_num + 2]
                .iter()
                .flat_map(|line| {
                    NUM_RE.captures_iter(line).filter_map(|cap| {
                        if let Some(num_match) = cap.get(0) {
                            if range_overlapping(&num_match.range(), &match_range) {
                                return Some(num_match.as_str().parse::<usize>().unwrap());
                            }
                        }
                        None
                    })
                })
                .collect::<Vec<usize>>();

            if gear_nums.len() > 1 {
                result += gear_nums.into_iter().product::<usize>();
            }
        }
    }

    result
}

fn range_overlapping(r1: &Range<usize>, r2: &Range<usize>) -> bool {
    cmp::max(r1.start, r2.start) < cmp::min(r1.end, r2.end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = fs::read_to_string("./data/day3.example1").unwrap();

        let result = process1(&input);

        assert_eq!(result, 4361);
    }

    #[test]
    fn test_example2() {
        let input = fs::read_to_string("./data/day3.example1").unwrap();

        let result = process2(&input);

        assert_eq!(result, 467835);
    }
}
