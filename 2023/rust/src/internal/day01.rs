pub fn process1(input: &str) -> u64 {
    let mut result = 0;

    for line in input.lines() {
        let digits = [
            find_digit(line, false, false),
            find_digit(line, false, true),
        ];
        result += String::from_iter(digits).parse::<u64>().unwrap();
    }

    result
}

pub fn process2(input: &str) -> u64 {
    let mut result = 0;

    for line in input.lines() {
        let digits = [find_digit(line, true, false), find_digit(line, true, true)];
        result += String::from_iter(digits).parse::<u64>().unwrap();
    }

    result
}

const DIGIT_SPELLED: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_digit(line: &str, spelled: bool, rev: bool) -> char {
    let digit_idx = (if rev { str::rfind } else { str::find })(line, |c: char| c.is_ascii_digit());
    let digit_value = digit_idx.map(|idx| {
        String::from(line.chars().nth(idx).unwrap())
            .parse()
            .unwrap()
    });
    if !spelled {
        return digit_value.unwrap();
    }
    let mut spelled_idx: Option<usize> = None;
    let mut spelled_value: Option<char> = None;
    for (idx, val) in DIGIT_SPELLED.iter().enumerate() {
        let line_idx = (if rev { str::rfind } else { str::find })(line, val);
        if line_idx.is_some()
            && (spelled_idx.is_none() || rev != (spelled_idx.unwrap() > line_idx.unwrap()))
        {
            spelled_idx = line_idx;
            spelled_value = (idx + 1).to_string().chars().next();
        }
    }

    match (digit_idx, spelled_idx) {
        (Some(_), None) => digit_value.unwrap(),
        (None, Some(_)) => spelled_value.unwrap(),
        (Some(digit_idx), Some(spelled_idx)) if rev != (digit_idx < spelled_idx) => {
            digit_value.unwrap()
        }
        (Some(digit_idx), Some(spelled_idx)) if rev != (digit_idx > spelled_idx) => {
            spelled_value.unwrap()
        }
        _ => panic!("Nope"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
    };
    const EXAMPLE_2: &str = indoc! {
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
    };

    #[test]
    fn test_example1() {
        let result = process1(EXAMPLE_1);

        assert_eq!(result, 142);
    }

    #[test]
    fn test_example2() {
        let result = process2(EXAMPLE_2);

        assert_eq!(result, 281);
    }
}