#![allow(clippy::unused_unit)]
#![allow(non_snake_case)]

use crate::*;

// https://adventofcode.com/2025/day/1


pub fn main_1() -> i32 {
    let lines = {
        open("day01/input.txt", "r").readlines()
    };
    let lines_len = lines.len();
    // preallocation, but in the end we did not need the full vec, even in part2
    let mut directions = vec!["R".to_string(); lines_len];
    let mut increments = vec![0; lines_len];
    let mut current = 50;
    let mut total = 0;
    let L = "L".to_string();
    let R = "R".to_string();
    for (count, line_iter) in enumerate(&lines) {
        let line = pstr::rstrip(&line_iter);
        let line_chars = line.chars().collect::<Vec<_>>();
        directions[count] = line_chars[0].to_string();
        increments[count] = int(&(line_chars[1..].iter().collect::<String>()));
        //match &directions[count] {
        if directions[count] == R {
            current += increments[count];
        } else if directions[count] == L {
            current -= increments[count];
        } else {
            panic!("Unknown direction");
        }
        current %= 100;
        if current == 0 {
            total += 1
        }
    }

    total
}


pub fn main_2() -> i32 {
    let lines = {
        open("day01/input.txt", "r").readlines()
    };
    let mut current = 50;
    let mut total = 0;
    let L = "L".to_string();
    let R = "R".to_string();
    for line_iter in &lines {
        let line = pstr::rstrip(line_iter);
        let line_chars = line.chars().collect::<Vec<_>>();
        let direction = line_chars[0].to_string();
        let increment = int::<i32>(&(line_chars[1..].iter().collect::<String>()));
        if direction == R {
            current += increment;
        } else if direction == L {
            current -= increment;
        } else {
            panic!("Unknown direction");
        }
        current %= 100;
        if current < 0 {
            // mod in rust i32 keeps the negative reminder
            current += 100;
        }
        if current == 0 {
            total += 1;
        }
        // zero crossings
        if direction == R && current < (increment % 100) && current != 0 {
            total += 1;
        }
        if direction == L && current > 100 - (increment % 100) {
            total += 1;
        }
        // full loops
        total += increment / 100;
    }

    total
}
