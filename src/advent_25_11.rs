#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2025/day/11
// 


pub fn main_1() -> usize {
    let input = {
        open("day11/input.txt", "r").readlines()
    };
    let mut nodes = dict();
    input.iter()
        .map(|row| row.split_once(": ").unwrap())
        .for_each(|pair| {
            nodes.insert(pair.0, pair.1.split(" ").collect::<Vec<_>>());
        });

    let mut stack = vec![nodes["you"][0]];
    for node in &nodes["you"][1..] {
        stack.push(node);
    }
    let end = "out";
    let mut total = 0;
    while let Some(node) = stack.pop() {
        if node == end {
            total += 1;
            continue;
        }
        for vs in &nodes[node] {
            stack.push(vs);
        }
    }

    total
}

