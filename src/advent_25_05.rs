#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2025/day/5
// 00:35:44 / 01:57:56


fn get_input() -> (Vec<(usize, usize)>, Vec<usize>) {
    let db = {
        open("day05/input.txt", "r").readlines()
    };
    let delim_pos = db.iter().position(|x| x.trim().is_empty()).unwrap();  // \n
    let ranges = map(
        |x| map(int::<usize>, x.split("-")).collect_tuple::<(usize, usize)>().unwrap(),
        &db[..delim_pos]
    ).collect::<Vec<(usize, usize)>>();
    (ranges, map(|x| int::<usize>(x), &db[delim_pos + 1..]).collect::<Vec<usize>>())
}


pub fn main_1() -> u32 {
    let (ranges, items) = get_input();

    let mut total = 0;
    for item in items{
        for (a, b) in &ranges {
            if item >= *a && item <= *b {
                total += 1;
                break;
            }
        }
    }

    total
}


pub fn main_2() -> usize {
    let (mut ranges, _) = get_input();

    // Brute force set union
    // let mut ids = set();
    // for (a, b) in &ranges {
    //     ids.extend(Set::from_iter(*a..=*b));
    // }
    // len(&ids)

    ranges.sort_by_key(|x| x.0);
    let mut repeat = true;
    while repeat {
        repeat = false;
        let mut idx = 0;
        let mut new = (0, 0);
        for (c, (a, b)) in enumerate(&ranges) {
            if let Some((left, right)) = ranges[c + 1..].iter().next() {
                if *left <= b {
                    repeat = true;
                    idx = c;
                    new = (a, b.max(*right));
                    break;
                }
            }
        };
        if repeat {
            ranges[idx] = new;
            ranges.remove(idx + 1);
        }
    }

    ranges.iter().map(|(a, b)| b - a + 1).sum()
}
