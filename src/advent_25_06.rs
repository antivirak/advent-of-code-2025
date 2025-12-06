#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2025/day/6
// 00:49:30 / 02:33:52


#[derive(Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}


fn load_input() -> (Vec<String>, Vec<Operation>, String) {
    let matrix_str = {
        open("day06/input.txt", "r").readlines()
    };
    let size = len(&matrix_str) - 1;
    let matrix = matrix_str[..size].to_vec();
    let ops = matrix_str[size].split_whitespace()
        .map(|x| if x == "+" {
            Operation::Add
        } else if x == "*" {
            Operation::Multiply
        } else {
            panic!("Unknown operation")
        })
        .collect::<Vec<_>>();

    (matrix, ops, matrix_str[size].clone())
}


pub fn main_1() -> u64 {
    let (matrix, ops, _) = load_input();
    let matrix = matrix.iter()
        .map(
            |x| x.split_whitespace()
            .map(int::<u64>)
            .collect::<Vec<_>>()
        ).collect::<Vec<Vec<_>>>();

    enumerate(&ops)
        .map(|(idx, op)| matrix.iter().map(|x| x[idx])
            .fold(match op {
                Operation::Add => 0,
                Operation::Multiply => 1,
            }, |acc, x| match op {
                Operation::Add => acc + x,
                Operation::Multiply => acc * x,
            })
        ).sum()
}

/*
// works for test data - asserts equal digits
pub fn main_2() -> u64 {
    let (matrix, ops) = load_input();

    let digits = matrix[0].len() / ops.len();
    enumerate(&ops)
        .map(|(idx, op)| {
            let row = matrix.iter().map(|x| x.chars().skip(idx * (digits + 1)).take(digits).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
            let transposed = (0..digits)
                .map(|i| row.iter().map(|inner| inner[i]).collect::<String>())
                .collect::<Vec<String>>();
            transposed
                .iter().map(|x| int::<u64>(x))
                .fold(match op {
                    Operation::Add => 0,
                    Operation::Multiply => 1,
                }, |acc, x| match op {
                    Operation::Add => acc + x,
                    Operation::Multiply => acc * x,
                })
        }).sum()
}
*/

pub fn main_2() -> u64 {
    let (matrix, ops, last_line) = load_input();
    let indices = {  // ensure indices immutable by using block
        let mut indices = last_line.chars().enumerate()
            .filter_map(|(i, c)| if c.is_whitespace() { None } else { Some(i) })
            .collect::<Vec<usize>>();
        indices.push(last_line.chars().count() + 1);
        indices
    };

    enumerate(&ops)
        .map(|(idx, op)| {
            let take = indices[idx + 1] - indices[idx] - 1;
            let row = matrix.iter().map(
                |x| x.chars().skip(indices[idx]).take(take).collect::<Vec<_>>()
            ).collect::<Vec<Vec<_>>>();
            // transpose
            (0..take)
                .map(|i| row.iter().map(|inner| inner[i]).collect::<String>())
                .map(|x| int::<u64>(&x))
                .fold(match op {
                    Operation::Add => 0,
                    Operation::Multiply => 1,
                }, |acc, x| match op {
                    Operation::Add => acc + x,
                    Operation::Multiply => acc * x,
                })
        }).sum()
}
