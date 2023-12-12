use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day2.task").unwrap();

    let result1 = process1(&input);

    println!("Result1: {result1}");

    let result2 = process2(&input);

    println!("Result2: {result2}");
}

lazy_static! {
    static ref GAME_RE: Regex = Regex::new(r"^Game (?P<game_id>[0-9]+): (?P<rounds>.*?)$").unwrap();
}

fn process1(input: &str) -> usize {
    const MAX_BALLS: [usize; 3] = [12, 13, 14];

    let mut result = 0;
    'games: for line in input.lines() {
        if !line.starts_with("Game ") {
            continue;
        }
        let capture = GAME_RE.captures(line).unwrap();
        let game_num = capture
            .name("game_id")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let rounds = capture.name("rounds").unwrap().as_str();

        let mut max_balls_used = [0, 0, 0];
        for round in rounds.split(';') {
            for balls in round.split(',') {
                let balls_separated: Vec<&str> = balls.trim().split(' ').collect();
                if balls_separated.len() < 2 {
                    continue;
                }

                let ball_amount = balls_separated[0].parse::<usize>().unwrap();
                let ball_color = balls_separated[1];
                let ball_color_idx = match ball_color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("Nope"),
                };
                max_balls_used[ball_color_idx] = cmp::max(
                    max_balls_used[ball_color_idx],
                    ball_amount,
                );
            }
        }

        for (idx, max_balls) in max_balls_used.into_iter().enumerate() {
            if max_balls > MAX_BALLS[idx] {
                continue 'games;
            }
        }

        result += game_num;
    }

    result
}

fn process2(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        if !line.starts_with("Game ") {
            continue;
        }
        let capture = GAME_RE.captures(line).unwrap();

        let rounds = capture.name("rounds").unwrap().as_str();

        let mut max_balls_used = [0, 0, 0];
        for round in rounds.split(';') {
            for balls in round.split(',') {
                let balls_separated: Vec<&str> = balls.trim().split(' ').collect();
                if balls_separated.len() < 2 {
                    continue;
                }

                let ball_amount = balls_separated[0].parse::<usize>().unwrap();
                let ball_color = balls_separated[1];
                let ball_color_idx = match ball_color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("Nope"),
                };
                max_balls_used[ball_color_idx] = cmp::max(
                    max_balls_used[ball_color_idx],
                    ball_amount,
                );
            }
        }

        result += max_balls_used[0] * max_balls_used[1] * max_balls_used[2];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    };

    #[test]
    fn test_example1() {
        let result = process1(EXAMPLE_1);

        assert_eq!(result, 8);
    }

    #[test]
    fn test_example2() {
        let result = process2(EXAMPLE_1);

        assert_eq!(result, 2286);
    }
}
