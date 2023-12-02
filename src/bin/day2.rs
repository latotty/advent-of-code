use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fs;

fn main() {
    let input = fs::read_to_string("./data/day2.task").unwrap();

    let result1 = process1(input.clone());

    println!("Result1: {result1}");

    let result2 = process2(input);

    println!("Result2: {result2}");
}

lazy_static! {
    static ref GAME_RE: Regex = Regex::new(r"^Game (?P<game_id>[0-9]+): (?P<rounds>.*?)$").unwrap();
}

fn process1(input: String) -> usize {
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
        for round in rounds.split(";") {
            for balls in round.split(",") {
                let balls_separated: Vec<&str> = balls.trim().split(" ").collect();
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
                max_balls_used[ball_color_idx] =
                    cmp::max(max_balls_used[ball_color_idx], ball_amount);
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

fn process2(input: String) -> usize {
    let mut result = 0;
    for line in input.lines() {
        if !line.starts_with("Game ") {
            continue;
        }
        let capture = GAME_RE.captures(line).unwrap();

        let rounds = capture.name("rounds").unwrap().as_str();

        let mut max_balls_used = [0, 0, 0];
        for round in rounds.split(";") {
            for balls in round.split(",") {
                let balls_separated: Vec<&str> = balls.trim().split(" ").collect();
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
                max_balls_used[ball_color_idx] =
                    cmp::max(max_balls_used[ball_color_idx], ball_amount);
            }
        }

        result += max_balls_used[0] * max_balls_used[1] * max_balls_used[2];
    }

    result
}

#[test]
fn test_example1() {
    let input = fs::read_to_string("./data/day2.example1").unwrap();

    let result = process1(input);

    assert_eq!(result, 8);
}

#[test]
fn test_example2() {
    let input = fs::read_to_string("./data/day2.example1").unwrap();

    let result = process2(input);

    assert_eq!(result, 2286);
}
