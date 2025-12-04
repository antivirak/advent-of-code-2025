#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2025/day/4
// 00:32:55 / 00:41:35


/// Return all 8 direct neighbor indices
fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
        (x + 1, y + 1),
    ]
}


fn get_grid() -> (Vec<Vec<i32>>, usize) {
    let input_map = {
        open("day04/input.txt", "r")
        .readlines()
        .iter()
        .map(|x| pstr::rstrip(x).chars().map(|c| c != '.').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>()
    };
    let grid_len = len(&input_map);  // assuming square grid

    // Add frame around - no need to check bounds later
    let mut grid = np::zeros((grid_len + 2, grid_len + 2), 0);
    for (i, j) in itertools::product(&(1..=grid_len), &(1..=grid_len)) {
        if input_map[i - 1][j - 1] {
            grid[i][j] = 1;
        }
    }

    (grid, grid_len)
}


pub fn main_1() -> u32 {
    let (grid, grid_len) = get_grid();

    let mut total = 0;
    for (i, j) in itertools::product(&(1..=grid_len), &(1..=grid_len)) {
        if grid[i][j] == 0 {
            continue;
        }
        // TODO cleanup
        let mut subtotal = 0;
        for idx in neighbors(i, j) {
            let (x, y) = idx;
            if grid[x][y] == 1 {
                subtotal += 1;
            }
        }
        if subtotal < 4 {
            total += 1;
        }
    }

    total
}


pub fn main_2() -> u32 {
    let (mut grid, grid_len) = get_grid();

    let mut total = 0;
    loop {
        let mut changed = false;
        for (i, j) in itertools::product(&(1..=grid_len), &(1..=grid_len)) {
            if grid[i][j] == 0 {
                continue;
            }
            let mut subtotal = 0;
            for idx in neighbors(i, j) {
                let (x, y) = idx;
                if grid[x][y] == 1 {
                    subtotal += 1;
                }
            }
            if subtotal < 4 {
                total += 1;
                grid[i][j] = 0;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    total
}
