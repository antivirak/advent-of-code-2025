#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2025/day/2
// 00:58:02 / 01:29:30

/*
pub fn main_1() -> u64 {
    // This would solve ony endpoints of intervals
    {
        open("day02/test_input.txt", "r")
        .readlines()[0]
        .split(",")
        .map(
            |x| x.split("-")  // pstr::split(x, "-")
            .collect::<Vec<&str>>().into_iter()
            // or lstrip 0
            .map(|x| int::<u64>(x).to_string())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>().into_iter()
        .map(|pair| {
            let a = &pair[0];  //.clone();
            let b = &pair[1];
            // no need to check for even len; might speed it up though
            // working with bytes would also be faster, numbers have guaranteed ASCII
            let mut subtotal = 0;
            for c in &[a, b] {
                let half = c.chars().count() / 2;
                let c_chars_vec = c.chars().collect::<Vec<_>>();
                print(&c);
                if c_chars_vec[..half] == c_chars_vec[half..] {
                    // I've already made this conversion upper; could store the result
                    subtotal += int::<u64>(&c);
                }
            }
            subtotal
        })
        .collect::<Vec<_>>().iter()
        .sum()
    }
}
*/


pub fn main_1() -> u64 {
    // bruteforce, fancy oneliner, but not satisfied with all the collect<Vec> calls
    // solvable by RegEx instead
    {
        open("day02/input.txt", "r")
        .readlines()[0]
        .split(",")
        .map(
            |x| x.split("-")
            .collect::<Vec<&str>>().into_iter()
            // or lstrip 0
            .map(int::<u64>)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>().into_iter()
        .map(|pair| {
            // no need to check for even len; might speed it up though
            // working with bytes would also be faster, numbers have guaranteed ASCII
            let mut subtotal = 0;
            for c in pair[0]..pair[1] + 1 {
                let c_string = c.to_string();
                let c_chars = c_string.chars();
                let half = c_chars.clone().count() / 2;
                let c_chars_vec = c_chars.collect::<Vec<_>>();
                if c_chars_vec[..half] == c_chars_vec[half..] {
                    subtotal += c;
                }
            }
            subtotal
        })
        .collect::<Vec<_>>().iter()
        .sum()
    }
}


pub fn main_2() -> u64 {
    {
        open("day02/input.txt", "r")
        .readlines()[0]
        .split(",")
        .map(
            |x| x.split("-")
            .collect::<Vec<&str>>().into_iter()
            // or lstrip 0
            .map(int::<u64>)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>().into_iter()
        .map(|pair| {
            let mut subtotal = 0;
            for c in pair[0]..pair[1] + 1 {
                let c_string = c.to_string();
                let c_chars = c_string.chars();
                let c_chars_len = c_chars.clone().count();
                for half in 1..c_chars_len / 2 + 1 {
                    if c_chars_len % half != 0 {
                        // now I need to check for "even" len
                        continue;
                    }
                    let c_chars_vec = c_chars.clone().collect::<Vec<char>>();
                    if c_chars_vec[..half].iter().cycle().take(c_chars_len).copied().collect::<Vec<_>>() == c_chars_vec {
                        subtotal += c;
                        break;
                    }
                }
            }
            subtotal
        })
        .collect::<Vec<_>>().iter()
        .sum()
    }
}
