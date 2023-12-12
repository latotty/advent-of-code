use std::collections::VecDeque;

pub fn process1(input: &str) -> u16 {
    let input = wall_input(input);

    let chars = input.chars().filter(|c| c != &'\n').collect::<Vec<char>>();
    let width = input.lines().next().unwrap().len();

    let mut flood_map = vec![Option::<u16>::None; chars.len()];
    let mut job_queue = VecDeque::<usize>::new();

    {
        let (start_idx, _) = chars
            .iter()
            .enumerate()
            .find(|(_, val)| val == &&'S')
            .expect("should have a start");
        flood_map[start_idx] = Some(0);
        job_queue.push_back(start_idx);
    }

    let directions: [i32; 4] = [-(width as i32), -1, 1, width as i32];
    while !job_queue.is_empty() {
        let current_idx = job_queue.pop_front().unwrap();
        let current_val = flood_map[current_idx].expect("should have value");
        let current_cmd = chars.get(current_idx).expect("should have a command");

        let cmds: [(Option<usize>, char); 5] = [
            if let Some(idx) = add_u_i32(current_idx, directions[0]) {
                (
                    Some(idx),
                    *chars.get(idx).unwrap_or(&'.'),
                )
            } else {
                (None, ' ')
            },
            if let Some(idx) = add_u_i32(current_idx, directions[1]) {
                (
                    Some(idx),
                    *chars.get(idx).unwrap_or(&'.'),
                )
            } else {
                (None, ' ')
            },
            (Some(current_idx), *current_cmd),
            if let Some(idx) = add_u_i32(current_idx, directions[2]) {
                (
                    Some(idx),
                    *chars.get(idx).unwrap_or(&'.'),
                )
            } else {
                (None, ' ')
            },
            if let Some(idx) = add_u_i32(current_idx, directions[3]) {
                (
                    Some(idx),
                    *chars.get(idx).unwrap_or(&'.'),
                )
            } else {
                (None, ' ')
            },
        ];
        let next_idxs = cmd_to_next(cmds);

        for next_idx in next_idxs.into_iter().flatten() {
            let new_val = match flood_map[next_idx] {
                Some(val) => {
                    if val <= current_val + 1 {
                        None
                    } else {
                        Some(current_val + 1)
                    }
                }
                None => Some(current_val + 1),
            };
            if let Some(new_val) = new_val {
                flood_map[next_idx] = Some(new_val);
                job_queue.push_back(next_idx);
            }
        }
    }

    // println!("{input}");
    // display_flood_map(width, &flood_map);

    *flood_map.iter().flatten().max().unwrap()
}

fn wall_input(input: &str) -> String {
    let top_wall = ".".repeat(input.lines().next().unwrap().len() + 2);
    let mut lines = input
        .lines()
        .map(|l| format!(".{l}."))
        .collect::<VecDeque<String>>();
    lines.push_front(String::from(&top_wall));
    lines.push_back(String::from(&top_wall));
    Vec::from(lines).join("\n")
}

pub fn display_flood_map(width: usize, flood_map: &[Option<u16>]) {
    for chunk in flood_map.chunks(width) {
        for e in chunk {
            print!(
                "{0: ^5}",
                if let Some(e) = e { *e as i32 } else { -1 }
            );
        }
        println!();
    }
}

fn add_u_i32(u: usize, i: i32) -> Option<usize> {
    if i.is_negative() {
        u.checked_sub(i.wrapping_abs() as u32 as usize)
    } else {
        u.checked_add(i as usize)
    }
}

fn cmd_to_next(cmds: [(Option<usize>, char); 5]) -> [Option<usize>; 4] {
    let mut next_directions = [None; 4];

    // N
    if let [(Some(next_idx), '|' | '7' | 'F'), _, (_, '|' | 'L' | 'J' | 'S'), _, _] = cmds {
        next_directions[0] = Some(next_idx);
    }
    // E
    if let [_, _, (_, '-' | 'L' | 'F' | 'S'), (Some(next_idx), '-' | 'J' | '7'), _] = cmds {
        next_directions[1] = Some(next_idx);
    }
    // S
    if let [_, _, (_, '|' | '7' | 'F' | 'S'), _, (Some(next_idx), '|' | 'L' | 'J')] = cmds {
        next_directions[2] = Some(next_idx);
    }
    // W
    if let [_, (Some(next_idx), '-' | 'L' | 'F'), (_, '-' | 'J' | '7' | 'S'), _, _] = cmds {
        next_directions[3] = Some(next_idx);
    }

    // dbg!(&cmds, &next_directions);

    next_directions
}

pub fn process2(input: &str) -> usize {
    let input = wall_input(input);

    let chars = input.chars().filter(|c| c != &'\n').collect::<Vec<char>>();
    let width = input.lines().next().unwrap().len();

    let mut wall_map = generate_wall_map(&chars, width)
        .iter()
        .cloned()
        .map(|b| Some(b as u8))
        .collect::<Vec<Option<u8>>>();

    let start_idx = get_wall_map_start_idx(&chars, width);

    find_main_loop_in_wall_map(&mut wall_map, width, start_idx);
    if std::env::var("DRAW_WALL_MAP").is_ok() {
        display_wall_map(width, &wall_map);
    }
    reduce_wall_map(&mut wall_map, width);

    if std::env::var("DRAW_WALL_MAP").is_ok() {
        display_wall_map(width, &wall_map);
    }

    wall_map
        .chunks(width * 3)
        .collect::<Vec<&[Option<u8>]>>()
        .chunks(3)
        .map(|lines| {
            let mut lines = lines.iter().map(|line| line.chunks(3));
            lines
                .next()
                .unwrap()
                .zip(lines.next().unwrap())
                .zip(lines.next().unwrap())
                .map(|((a, b), c)| {
                    [a, b, c]
                        .concat()
                        .iter()
                        .any(|e| matches!(e, Some(2) | None))
                })
                .filter(|b| !*b)
                .count()
        })
        .sum()
}

fn get_wall_map_start_idx(chars: &[char], width: usize) -> usize {
    let start_idx = {
        let (start_idx, _) = chars
            .iter()
            .enumerate()
            .find(|(_, val)| val == &&'S')
            .expect("should have a start");
        let start_y = start_idx / width;
        let start_x = start_idx % width;
        let wall_width = width * 3;
        let wall_y = start_y * 3 + 1;
        let wall_x = start_x * 3 + 1;
        wall_y * wall_width + wall_x
    };
    start_idx
}

fn find_main_loop_in_wall_map(wall_map: &mut [Option<u8>], width: usize, start_idx: usize) {
    let mut job_queue = VecDeque::<usize>::new();
    job_queue.push_back(start_idx);
    *wall_map.get_mut(start_idx).unwrap() = Some(2);

    let directions: [i32; 4] = [-(width as i32) * 3, -1, 1, width as i32 * 3];
    while let Some(curr_idx) = job_queue.pop_front() {
        for dir in directions {
            let next_idx = add_u_i32(curr_idx, dir);
            if let Some(next_idx) = next_idx {
                let target = match wall_map.get(next_idx) {
                    Some(Some(1)) => true,
                    Some(Some(_)) => false,
                    _ => false,
                };
                if target {
                    let val = wall_map.get_mut(next_idx).unwrap();
                    *val = Some(2);
                    job_queue.push_back(next_idx);
                }
            }
        }
    }
}

fn reduce_wall_map(wall_map: &mut [Option<u8>], width: usize) {
    let mut job_queue = VecDeque::<usize>::new();
    job_queue.push_back(0);
    *wall_map.get_mut(0).unwrap() = None;

    let directions: [i32; 4] = [-(width as i32) * 3, -1, 1, width as i32 * 3];
    while let Some(curr_idx) = job_queue.pop_front() {
        for dir in directions {
            let next_idx = add_u_i32(curr_idx, dir);
            if let Some(next_idx) = next_idx {
                let target = match wall_map.get(next_idx) {
                    Some(Some(0 | 1)) => true,
                    Some(Some(2)) => {
                        directions
                            .iter()
                            .filter(|dir| {
                                let idx = add_u_i32(next_idx, **dir);
                                if let Some(idx) = idx {
                                    return matches!(wall_map.get(idx), Some(Some(_)));
                                }
                                false
                            })
                            .count()
                            <= 1
                    }
                    _ => false,
                };
                if target {
                    let val = wall_map.get_mut(next_idx).unwrap();
                    *val = None;
                    job_queue.push_back(next_idx);
                }
            }
        }
    }
}

fn generate_wall_map(chars: &Vec<char>, width: usize) -> Vec<bool> {
    let mut wall_map = vec![false; chars.len() * 9];
    for (idx, char) in chars.iter().enumerate() {
        let x = idx % width;
        let y = idx / width;
        let walls = cmd_to_walls(char);

        (0..9).for_each(|wall_idx| {
            // dbg!(x, y, wall_idx % 3, wall_idx / 3);
            let x = x * 3 + wall_idx % 3;
            let y = y * 3 + wall_idx / 3;
            // dbg!(wall_idx, x, y, x + (width * 3) * y);

            let wall_ref = wall_map.get_mut(x + (width * 3) * y);
            if let Some(wall_ref) = wall_ref {
                *wall_ref = walls[wall_idx];
            }
        });
    }
    wall_map
}

pub fn display_wall_map(width: usize, flood_map: &[Option<u8>]) {
    for chunk in flood_map.chunks(width * 3) {
        for e in chunk {
            // █▓▒░
            print!(
                "{}",
                match *e {
                    Some(2) => '█',
                    Some(1) => '▓',
                    Some(0) => '░',
                    None => ' ',
                    _ => panic!("nope"),
                }
            );
        }
        println!();
    }
}

const WALLS: [[bool; 9]; 8] = [
    [
        // '|'
        false, true, false, false, true, false, false, true, false,
    ],
    [
        // '-'
        false, false, false, true, true, true, false, false, false,
    ],
    [
        // 'L'
        false, true, false, false, true, true, false, false, false,
    ],
    [
        // 'J'
        false, true, false, true, true, false, false, false, false,
    ],
    [
        // '7'
        false, false, false, true, true, false, false, true, false,
    ],
    [
        // 'F'
        false, false, false, false, true, true, false, true, false,
    ],
    [
        // 'S'
        false, true, false, true, true, true, false, true, false,
    ],
    [
        false, false, false, false, false, false, false, false, false,
    ],
];

fn cmd_to_walls(cmd: &char) -> &'static [bool; 9] {
    match cmd {
        '|' => &WALLS[0],
        '-' => &WALLS[1],
        'L' => &WALLS[2],
        'J' => &WALLS[3],
        '7' => &WALLS[4],
        'F' => &WALLS[5],
        'S' => &WALLS[6],
        _ => &WALLS[7],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TASK: &str = include_str!("../../data/day10.task");

    #[rstest]
    #[case::c1(&[
        ".....",
        ".S-7.",
        ".|.|.",
        ".L-J.",
        "....."
    ].join("\n"), 4)]
    #[case::c2(&[
        "..F7.",
        ".FJ|.",
        "SJ.L7",
        "|F--J",
        "LJ..."
    ].join("\n"), 8)]
    fn test_process1_example(#[case] input: &str, #[case] expected: u16) {
        let result = process1(input);

        assert_eq!(result, expected);
    }

    // #[ignore = "not yet"]
    #[test]
    fn test_process1_task() {
        let result = process1(TASK);

        assert_eq!(result, 7030);
    }

    #[rstest]
    #[case::c1(&[
        "...........",
        ".S-------7.",
        ".|F-----7|.",
        ".||.....||.",
        ".||.....||.",
        ".|L-7.F-J|.",
        ".|..|.|..|.",
        ".L--J.L--J.",
        "...........",
    ].join("\n"), 4)]
    #[case::c2(&[
        ".F----7F7F7F7F-7....",
        ".|F--7||||||||FJ....",
        ".||.FJ||||||||L7....",
        "FJL7L7LJLJ||LJ.L-7..",
        "L--J.L7...LJS7F-7L7.",
        "....F-J..F7FJ|L7L7L7",
        "....L7.F7||L7|.L7L7|",
        ".....|FJLJ|FJ|F7|.LJ",
        "....FJL-7.||.||||...",
        "....L---J.LJ.LJLJ...",
    ].join("\n"), 8)]
    #[case::c3(&[
        "FF7FSF7F7F7F7F7F---7",
        "L|LJ||||||||||||F--J",
        "FL-7LJLJ||||||LJL-77",
        "F--JF--7||LJLJ7F7FJ-",
        "L---JF-JLJ.||-FJLJJ7",
        "|F|F-JF---7F7-L7L|7|",
        "|FFJF7L7F-JF7|JL---7",
        "7-L-JL7||F7|L7F-7F7|",
        "L.L7LFJ|||||FJL7||LJ",
        "L7JLJL-JLJLJL--JLJ.L",
    ].join("\n"), 10)]
    fn test_process2_example(#[case] input: &str, #[case] expected: usize) {
        let result = process2(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(&[
        ".",
    ].join("\n"), vec![
        false, false, false,
        false, false, false,
        false, false, false,
    ])]
    #[case::c2(&[
        "S",
    ].join("\n"), vec![
        false, true, false,
        true, true, true,
        false, true, false,
    ])]
    #[case::c2(&[
        "SS",
        "SS"
    ].join("\n"), vec![
        false, true, false, false, true, false,
        true, true, true, true, true, true,
        false, true, false, false, true, false,
        false, true, false, false, true, false,
        true, true, true, true, true, true,
        false, true, false, false, true, false,
    ])]
    fn generate_wall_map_test(#[case] input: &str, #[case] expected: Vec<bool>) {
        let chars = input.chars().filter(|c| c != &'\n').collect::<Vec<char>>();
        let width = input.lines().next().unwrap().len();
        let result = generate_wall_map(&chars, width);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::c1(&[
        "S",
    ].join("\n"), 4)]
    #[case::c1(&[
        "..",
        ".S",
    ].join("\n"), 28)]
    fn get_wall_map_start_idx_test(#[case] input: &str, #[case] expected: usize) {
        let chars = input.chars().filter(|c| c != &'\n').collect::<Vec<char>>();
        let width = input.lines().next().unwrap().len();
        let result = get_wall_map_start_idx(&chars, width);

        assert_eq!(result, expected);
    }
}
