#![allow(clippy::unused_unit)]
#![allow(non_snake_case)]

use std::collections::VecDeque;
use crate::*;

// https://adventofcode.com/2025/day/7
// 02:49:24 / 05:04:48


/*
/// This solves entirely different problem
pub fn main_1() -> usize {
    let hall = {
        open("day07/test_input.txt", "r").readlines()
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>()
    };

    let S = 'S';
    let DOT = '.';
    let SPLIT = '^';
    // assuming start on first line
    // let start = (0, list::index(hall[0], S));
    let start = (1, hall[0].iter().position(|&x| x == S).unwrap());

    let mut stack = vec![];
    stack.push(start);
    let mut beams = set();
    let end_vertical = len(&hall) - 1;
    while let Some(current) = stack.pop() {
        if current.0 == end_vertical {
            beams.insert(current.1);
            continue;
        }

        if hall[current.0][current.1] == DOT {
            stack.push((current.0 + 1, current.1));
            continue;
        }

        if hall[current.0][current.1] == SPLIT {
            // bounds check?
            stack.push((current.0 + 1, current.1 - 1));
            stack.push((current.0 + 1, current.1 + 1));
        }
    }

    len(&beams)
}
    */

pub fn main_1() -> usize {
    let hall = {
        open("day07/input.txt", "r").readlines()
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>()
    };

    let S = 'S';
    let DOT = '.';
    let SPLIT = '^';
    // assuming start on first line
    // let start = (0, list::index(hall[0], S));
    let start = (1, hall[0].iter().position(|&x| x == S).unwrap());

    let mut stack = vec![];
    stack.push(start);
    let mut used_splitters = set();
    let mut visited = set::<(_, _)>();
    let end_vertical = len(&hall) - 1;
    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        if hall[current.0][current.1] == DOT {
            if current.0 != end_vertical {
                stack.push((current.0 + 1, current.1));
            }
            continue;
        }

        if hall[current.0][current.1] == SPLIT {
            used_splitters.insert((current.0, current.1));
            // bounds check?
            stack.push((current.0 + 1, current.1 - 1));
            stack.push((current.0 + 1, current.1 + 1));
        }
    }

    len(&used_splitters)
}

/*
// This strategy works on test input probably just by accident
pub fn main_2() -> usize {
    let hall = {
        open("day07/test_input.txt", "r").readlines()
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>()
    };

    let S = 'S';
    let DOT = '.';
    let SPLIT = '^';
    let start = (1, hall[0].iter().position(|&x| x == S).unwrap());

    let mut stack = vec![];
    stack.push(start);
    let mut beams = set::<(_, _)>();
    let mut visited = set::<(_, _)>();
    let end_vertical = len(&hall) - 1;
    while let Some(current) = stack.pop() {
        if visited.contains(&current) {
            continue;
        }

        visited.insert(current);

        if current.0 % 2 == 1 {
            beams.insert(current);
        }

        if hall[current.0][current.1] == DOT {
            if current.0 != end_vertical {
                stack.push((current.0 + 1, current.1));
            }
            continue;
        }

        if hall[current.0][current.1] == SPLIT {
            stack.push((current.0 + 1, current.1 - 1));
            stack.push((current.0 + 1, current.1 + 1));
        }
    }

    len(&beams) - 1  // Substract the starting beam not created by splitter
}
    */

pub fn main_2() -> usize {
    let hall = {
        open("day07/input.txt", "r").readlines()
        .iter()
        .map(|row| row.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>()
    };

    let S = 'S';
    let DOT = '.';
    let SPLIT = '^';
    let start = (1, hall[0].iter().position(|&x| x == S).unwrap());

    let mut q = VecDeque::new();  // BFS
    q.push_front((start, 1));
    let mut visited = dict::<(_, _), _>();
    let end_vertical = len(&hall) - 1;
    while let Some(current_pair) = q.pop_front() {
        let (current, timelines) = current_pair;
        let item_is_dot = hall[current.0][current.1] == DOT;
        let item_is_split = hall[current.0][current.1] == SPLIT;
        if let std::collections::hash_map::Entry::Vacant(entry) = visited.entry(current) {
            entry.insert(timelines);
        } else {
            // do not push new items when already visited,
            // but propagate the number of visits (which is easy for BFS)
            *visited.get_mut(&current).unwrap() += timelines;
            if let Ok(idx) = q.binary_search_by_key(&(current.0 + 1, current.1), |&(a, _)| a) {
                if item_is_dot {
                    if current.0 != end_vertical {
                        q[idx] = ((current.0 + 1, current.1), q[idx].1 + timelines);
                    }
                } else if item_is_split {
                    q[idx] = ((current.0 + 1, current.1 - 1), q[idx].1 + timelines);
                    q[idx + 1] = ((current.0 + 1, current.1 + 1), q[idx].1 + timelines);
                }
            }
            continue;
        }

        if item_is_dot {
            if current.0 != end_vertical {
                q.push_back(((current.0 + 1, current.1), timelines));
            }
            continue;
        }

        if item_is_split {
            q.push_back(((current.0 + 1, current.1 - 1), timelines));
            q.push_back(((current.0 + 1, current.1 + 1), timelines));
        }
    }

    let mut total = 0;
    for idx in 0..len(&hall[0]) {
        if let Some(item) = visited.get(&(end_vertical, idx)) {
            total += item;
        }
    }

    total
}
