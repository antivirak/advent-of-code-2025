#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2025/day/9
// 00:27:18 / 07:54:37


pub fn main_1() -> usize {
    let coords = {
        open("day09/input.txt", "r").readlines()
        .iter()
        .map(|row| row.split(",")
            //.sorted()
            .map(int).collect::<Vec<usize>>())
        .collect::<Vec<Vec<_>>>()
    };
    let coords_no = len(&coords);

    let mut total = 0;
    for x in 0..coords_no - 1 {
        for y in x + 1..coords_no {
            let a = np::substract(
                coords[x].clone(), coords[y].clone()
            );
            //.product());
            total = max(total, (a[0]+1)*(a[1]+1));
        }
    }

    total
}


pub fn main_2() -> usize {
    42
}
