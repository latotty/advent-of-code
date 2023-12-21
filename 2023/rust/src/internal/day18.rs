use std::{cmp, collections::VecDeque, ops::Range, sync::{Arc, atomic::{AtomicUsize, AtomicIsize}}};

use rayon::{slice::ParallelSliceMut, iter::ParallelIterator};

pub fn process1(input: &str) -> usize {
    // process1_graph::<true>(input)
    process1_str(input)
}

pub fn process1_graph_par<const DRAW: bool>(input: &str) -> usize {
    let graph: Arc<Vec<(isize, isize, bool)>> = Arc::from(build_graph(input));

    if DRAW {
        println!(
            "Start:\n{}\n",
            draw_graph::<true>(&graph)
        );
    }

    let in_right = graph[0].2;
    let area = AtomicIsize::new(0);

    let mut graph: Arc<Vec<(isize, isize, bool)>> = Arc::from(graph);
    let mut next_graph: Vec<(isize, isize, bool)> = (*graph).clone();
    loop {
        next_graph.par_chunks_mut(4).for_each_with(graph.clone(), |graph, chunk| {
            if chunk.len() < 4 {
                return;
            }
            if let Some((mut nodes, area_calc)) = reduce_graph_chunk::<DRAW>(
                chunk,
                graph,
                in_right,
            ) {
                area.fetch_add(area_calc, std::sync::atomic::Ordering::AcqRel);
                nodes.resize_with(4, || (isize::MIN, isize::MAX, false));
                chunk.swap_with_slice(&mut nodes);
            }
        });
        next_graph.retain(|e| e != &(isize::MIN, isize::MAX, false));
        next_graph.rotate_right(1);

        (next_graph, graph) = (next_graph.clone(), Arc::from(next_graph));

        dbg!(next_graph.len());

        if next_graph.len() <= 4 {
            break;
        }
    }

    let mut area = area.load(std::sync::atomic::Ordering::Relaxed);

    let (_, x_range, y_range, _) = get_graph_working_area(next_graph[0..4].try_into().unwrap());
    area += ((x_range.len()) * (y_range.len())) as isize;

    area as usize
}

pub fn process1_graph<const DRAW: bool>(input: &str) -> usize {
    let mut graph = build_graph(input);

    if DRAW {
        println!(
            "Start:\n{}\n",
            draw_graph::<true>(&graph)
        );
    }

    let in_right = graph[0].2;
    let mut area = 0isize;
    while graph.len() > 4 {
        if let Some((mut nodes, area_calc)) = reduce_graph_chunk::<DRAW>(
            &graph[graph.len() - 4..],
            &graph,
            in_right,
        ) {
            area += area_calc;
            graph.drain(graph.len() - 4..);
            graph.append(&mut nodes);
        } else {
            graph.rotate_right(1);
        }
    }
    if DRAW {
        println!(
            "Step ({area}):\n{}\n",
            draw_graph::<true>(&graph)
        );
    }

    let (_, x_range, y_range, _) = get_graph_working_area(graph[0..4].try_into().unwrap());
    area += ((x_range.len()) * (y_range.len())) as isize;

    if DRAW {
        println!("Final ({area})");
    }

    area as usize
}

fn reduce_graph_chunk<const DRAW: bool>(
    nodes: &[(isize, isize, bool)],
    graph: &[(isize, isize, bool)],
    in_right: bool,
) -> Option<(Vec<(isize, isize, bool)>, isize)> {
    let (vertical, x_range, y_range, orientation) = get_graph_working_area(nodes);
    if (/* invalid vertical */vertical
        && nodes[1].0 < cmp::max(nodes[0].0, nodes[3].0)
        && nodes[1].0 > cmp::min(nodes[0].0, nodes[3].0))
        || (/* invalid horizontal */!vertical
            && nodes[1].1 < cmp::max(nodes[0].1, nodes[3].1)
            && nodes[1].1 > cmp::min(nodes[0].1, nodes[3].1))
        || (/* not empty */graph
            .iter()
            .filter(|e| !nodes.contains(e))
            .any(|(x, y, _)| x_range.contains(x) && y_range.contains(y)))
    {
        return None;
    }
    let inside = in_right == nodes[1].2;
    let area = if vertical && !inside {
        -2 * (x_range.len() as isize) + 2
    } else if vertical && inside {
        ((x_range.len() - 1) * (y_range.len())) as isize
    } else {
        return None;
        // panic!("No area?");
    };

    if DRAW {
        // println!(
        //     "Step ({area}):\n{}\n",
        //     draw_graph::<true>(&graph)
        // );
        println!(
            "Current: ({area})\n{}\n",
            draw_graph::<false>(&nodes)
        );
    }
    if (vertical && nodes[0].0 == nodes[3].0) || (!vertical && nodes[0].1 == nodes[3].1) {
        return Some((Vec::new(), area));
    }

    // else {
    //     area += (x_range.len() * (y_range.len() - 1)) as isize * area_mod;
    // }

    // if ver and 0.x == 3.x -> rem all or if hor and 0.y == 3.y -> rem all

    if vertical {
        if orientation
        /* left */
        {
            if nodes[0].0 < nodes[3].0 {
                // remove all
                // add 0 and topleft
                return Some((
                    vec![nodes[0], (x_range.start, y_range.start, nodes[1].2)],
                    area,
                ));
            } else {
                // remove all
                // add bottomleft and 3
                return Some((
                    vec![
                        (
                            x_range.start,
                            y_range.end - 1,
                            nodes[1].2,
                        ),
                        nodes[3],
                    ],
                    area,
                ));
            }
        } else if nodes[0].0 > nodes[3].0 {
            // remove all
            // add 0 and topright
            return Some((
                vec![
                    nodes[0],
                    (
                        x_range.end - 1,
                        y_range.start,
                        nodes[1].2,
                    ),
                ],
                area,
            ));
        } else {
            // remove all
            // add topright and 3
            return Some((
                vec![
                    (
                        x_range.end - 1,
                        y_range.start,
                        nodes[1].2,
                    ),
                    nodes[3],
                ],
                area,
            ));
        }
    }
    None
}

fn get_graph_working_area(
    nodes: &[(isize, isize, bool)],
) -> (bool, Range<isize>, Range<isize>, bool) {
    if nodes[1].0 == nodes[2].0 {
        // vertical
        let width = cmp::min(
            nodes[0].0.abs_diff(nodes[1].0),
            nodes[2].0.abs_diff(nodes[3].0),
        );

        let left = nodes[0].0 < nodes[1].0;

        let top_left_x = if left {
            nodes[1].0 - width as isize
        } else {
            nodes[1].0
        };
        let top_left_y = cmp::min(nodes[1].1, nodes[2].1);

        let bottom_right_x = if left {
            nodes[1].0
        } else {
            nodes[1].0 + width as isize
        };
        let botton_right_y = cmp::max(nodes[1].1, nodes[2].1);

        return (
            true,
            top_left_x..bottom_right_x + 1,
            top_left_y..botton_right_y + 1,
            left,
        );
    }

    // horizontal
    let height = cmp::min(
        nodes[0].1.abs_diff(nodes[1].1),
        nodes[2].1.abs_diff(nodes[3].1),
    );

    let top = nodes[0].1 < nodes[1].1;

    let top_left_x = cmp::min(nodes[1].0, nodes[2].0);
    let top_left_y = if top {
        nodes[1].1 - height as isize
    } else {
        nodes[1].1
    };

    let botton_right_x = cmp::max(nodes[1].0, nodes[2].0);
    let botton_right_y = if top {
        nodes[1].1
    } else {
        nodes[1].1 + height as isize
    };

    (
        false,
        top_left_x..botton_right_x + 1,
        top_left_y..botton_right_y + 1,
        top,
    )
}

pub fn process1_str(input: &str) -> usize {
    let size_data = get_size(input);

    let hole = draw_hole(input, &size_data);
    // println!("{}", &hole);
    let reduced_hole = reduce_hole_map(&hole);
    // println!("{}", &reduced_hole);

    count_area(&reduced_hole)
}

pub fn process2(_input: &str) -> u64 {
    0
}

fn build_graph(input: &str) -> Vec<(isize, isize, bool)> {
    let lines_count = input.lines().count();
    let (mut x, mut y, mut last_cmd, mut last_right) = (0, 0, None, None);
    let mut res = vec![];
    input
        .lines()
        .cycle()
        .take(lines_count + 1)
        .for_each(|line| {
            let mut split = line.split(' ');
            let cmd = split.next().unwrap();
            let length = split.next().unwrap().parse::<isize>().unwrap();

            if let Some(last_cmd) = last_cmd {
                let right = match (last_cmd, cmd) {
                    ("R", "D") => true,
                    ("D", "L") => true,
                    ("L", "U") => true,
                    ("U", "R") => true,

                    ("D", "R") => false,
                    ("R", "U") => false,
                    ("U", "L") => false,
                    ("L", "D") => false,

                    // ("L", "L") | ("R", "R") | ("D", "D") | ("U", "U") => last_right.unwrap(),
                    _ => panic!("invalid cmds: {last_cmd} {cmd}"),
                };
                last_right = Some(right);
                res.push((x, y, right));
            }
            last_cmd = Some(cmd);

            match cmd {
                "U" => {
                    y -= length;
                }
                "L" => {
                    x -= length;
                }
                "D" => {
                    y += length;
                }
                "R" => {
                    x += length;
                }
                _ => panic!("invalid cmd: {cmd}"),
            }
        });
    res.rotate_right(1);
    res
}

fn draw_graph<const LAST: bool>(graph: &[(isize, isize, bool)]) -> String {
    let size_data = get_graph_size(graph);
    let mut result = (0..size_data.height)
        .map(|_| ".".repeat(size_data.width))
        .collect::<Vec<String>>()
        .join("\n");

    let (mut x, mut y) = (
        graph[0].0 - size_data.shift_x,
        graph[0].1 - size_data.shift_y,
    );

    let str_width = size_data.width as isize + 1;

    graph
        .iter()
        .cycle()
        .take(graph.len() + if LAST { 1 } else { 0 })
        .for_each(|(nx, ny, _)| {
            let (nx, ny) = (
                nx - size_data.shift_x,
                ny - size_data.shift_y,
            );
            if x != nx {
                result.replace_range(
                    (y * str_width + cmp::min(x, nx)) as usize
                        ..=(y * str_width + cmp::max(x, nx)) as usize,
                    &"#".repeat(x.abs_diff(nx) + 1),
                );
            }
            if y != ny {
                for y in cmp::min(y, ny)..=cmp::max(y, ny) {
                    result.replace_range(
                        (y * str_width + x) as usize..=(y * str_width + x) as usize,
                        "#",
                    );
                }
            }
            x = nx;
            y = ny;
        });

    graph.iter().for_each(|(x, y, right)| {
        let (x, y) = (
            x - size_data.shift_x,
            y - size_data.shift_y,
        );
        result.replace_range(
            (y * str_width + x) as usize..=(y * str_width + x) as usize,
            if *right { "R" } else { "L" },
        );
    });

    result
}

fn get_graph_size(graph: &[(isize, isize, bool)]) -> InputSizeData {
    let minx = graph.iter().min_by_key(|(x, _, _)| x).unwrap().0;
    let maxx = graph.iter().max_by_key(|(x, _, _)| x).unwrap().0;
    let miny = graph.iter().min_by_key(|(_, y, _)| y).unwrap().1;
    let maxy = graph.iter().max_by_key(|(_, y, _)| y).unwrap().1;

    InputSizeData {
        width: (maxx - minx + 1) as usize,
        height: (maxy - miny + 1) as usize,
        shift_x: minx,
        shift_y: miny,
    }
}

fn count_area(input: &str) -> usize {
    input.chars().filter(|c| c == &'#' || c == &'.').count()
}

fn reduce_hole_map(input: &str) -> String {
    let width = input.lines().next().unwrap().len();

    let mut result = input.chars().collect::<Vec<char>>();

    let mut job_queue = VecDeque::<usize>::new();
    job_queue.push_back(0);
    result[0] = ' ';

    let directions: [isize; 4] = [-(width as isize + 1), -1, 1, width as isize + 1];
    while let Some(curr_idx) = job_queue.pop_front() {
        for dir in directions {
            let next_idx = curr_idx.checked_add_signed(dir);
            if let Some(next_idx) = next_idx {
                if matches!(result.get(next_idx), Some('.')) {
                    result[next_idx] = ' ';
                    job_queue.push_back(next_idx);
                }
            }
        }
    }

    result.iter().collect::<String>()
}

fn draw_hole(input: &str, size_data: &InputSizeData) -> String {
    let mut result = (0..size_data.height + 2)
        .map(|_| ".".repeat(size_data.width + 2))
        .collect::<Vec<String>>()
        .join("\n");

    let (mut x, mut y) = (
        size_data.shift_x.abs() + 1,
        size_data.shift_y.abs() + 1,
    );

    let str_width = size_data.width as isize + 1 + 2;

    input.lines().for_each(|line| {
        let mut split = line.split(' ');
        let cmd = split.next().unwrap();
        let length = split.next().unwrap().parse::<isize>().unwrap();

        match cmd {
            "U" => {
                for y in y - length..y + 1 {
                    result.replace_range(
                        (y * str_width + x) as usize..(y * str_width + x + 1) as usize,
                        "#",
                    );
                }
                y -= length;
            }
            "L" => {
                result.replace_range(
                    (y * str_width + x - length) as usize..(y * str_width + x + 1) as usize,
                    &"#".repeat(length as usize + 1),
                );
                x -= length;
            }
            "D" => {
                for y in y..y + length {
                    result.replace_range(
                        (y * str_width + x) as usize..(y * str_width + x + 1) as usize,
                        "#",
                    );
                }
                y += length;
            }
            "R" => {
                result.replace_range(
                    (y * str_width + x) as usize..(y * str_width + x + length + 1) as usize,
                    &"#".repeat(length as usize + 1),
                );
                x += length;
            }
            _ => panic!("invalid cmd: {cmd}"),
        }
    });

    result
}

#[derive(Debug, PartialEq, Eq)]
struct InputSizeData {
    width: usize,
    height: usize,
    shift_x: isize,
    shift_y: isize,
}
fn get_size(input: &str) -> InputSizeData {
    let (mut minx, mut maxx, mut miny, mut maxy, mut x, mut y) = (0, 0, 0, 0, 0, 0);
    input.lines().for_each(|line| {
        let mut split = line.split(' ');
        let cmd = split.next().unwrap();
        let length = split.next().unwrap().parse::<isize>().unwrap();

        match cmd {
            "U" => {
                y -= length;
                miny = cmp::min(miny, y);
            }
            "L" => {
                x -= length;
                minx = cmp::min(minx, x);
            }
            "D" => {
                y += length;
                maxy = cmp::max(maxy, y);
            }
            "R" => {
                x += length;
                maxx = cmp::max(maxx, x);
            }
            _ => panic!("invalid cmd: {cmd}"),
        }
    });
    assert_eq!((x, y), (0, 0));
    InputSizeData {
        width: (maxx - minx + 1) as usize,
        height: (maxy - miny + 1) as usize,
        shift_x: minx,
        shift_y: miny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TASK: &str = include_str!("../../data/day18.task");
    const EXAMPLE_1: &str = indoc::indoc! {
        "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"
    };

    #[rstest]
    #[case::example(EXAMPLE_1, 62)]
    #[case::task(TASK, 40761)]
    fn process1_str_test(#[case] input: &str, #[case] expected: usize) {
        let result = process1_str(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::example(EXAMPLE_1, 62)]
    #[case::example_inv(indoc::indoc! {
        "L 6 (#70c710)
        U 5 (#0dc571)
        R 2 (#5713f0)
        U 2 (#d2c081)
        L 2 (#59c680)
        U 2 (#411b91)
        R 5 (#8ceee2)
        D 2 (#caa173)
        R 1 (#1b58a2)
        D 2 (#caa171)
        L 2 (#7807d2)
        D 3 (#a77fa3)
        R 2 (#015232)
        D 2 (#7a21e3)"
    }, 62)]
    // #[case::task(TASK, 40761)]
    fn process1_graph_test(#[case] input: &str, #[case] expected: usize) {
        let result = process1_graph::<false>(input);

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case::example(EXAMPLE_1, 62)]
    #[case::example_inv(indoc::indoc! {
        "L 6 (#70c710)
        U 5 (#0dc571)
        R 2 (#5713f0)
        U 2 (#d2c081)
        L 2 (#59c680)
        U 2 (#411b91)
        R 5 (#8ceee2)
        D 2 (#caa173)
        R 1 (#1b58a2)
        D 2 (#caa171)
        L 2 (#7807d2)
        D 3 (#a77fa3)
        R 2 (#015232)
        D 2 (#7a21e3)"
    }, 62)]
    // #[case::task(TASK, 40761)]
    fn process1_graph_par_test(#[case] input: &str, #[case] expected: usize) {
        let result = process1_graph_par::<false>(input);

        assert_eq!(result, expected);
    }

    // #[rstest]
    // #[case::c1(EXAMPLE_1, 145)]
    // fn process2_test(#[case] input: &str, #[case] expected: u64) {
    //     let result = process2(input);

    //     assert_eq!(result, expected);
    // }

    #[rstest]
    #[case::example(EXAMPLE_1, InputSizeData { width: 7, height: 10, shift_x: 0, shift_y: 0 })]
    #[case::task(TASK, InputSizeData { width: 373, height: 347, shift_x: 0, shift_y: -144 })]
    #[case::c01(indoc::indoc! {
        "R 5 (#000000)
        D 2 (#000000)
        R 1 (#000000)
        D 1 (#000000)
        L 5 (#000000)
        U 1 (#000000)
        L 1 (#000000)
        U 2 (#000000)"
    }, InputSizeData { width: 7, height: 4, shift_x: 0, shift_y: 0 })]
    #[case::c02(indoc::indoc! {
        "R 5 (#000000)
        U 5 (#000000)
        L 8 (#000000)
        D 5 (#000000)
        R 3 (#000000)"
    }, InputSizeData { width: 9, height: 6, shift_x: -3, shift_y: -5 })]
    fn get_size_test(#[case] input: &str, #[case] expected: InputSizeData) {
        let result = get_size(input);

        assert_eq!(result, expected);
    }
}
