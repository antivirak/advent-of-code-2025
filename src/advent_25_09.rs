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
            total = max(total, (a[0] + 1) * (a[1] + 1));
        }
    }

    total
}


pub fn main_2() -> usize {
    return 42;

    let path_vertices = {
        open("day09/input.txt", "r").readlines()
        .iter()
        .map(|row| row.split(",")
            //.sorted()
            .map(int).collect::<Vec<usize>>())
        .collect::<Vec<Vec<_>>>()
    };
    let coords_no = len(&coords);

    let mut total = 0;

    poly_v = np.asarray(path_vertices, int)
    poly = path.Path(poly_v, closed=True)

    for x in 0..coords_no {
        for y in x + 1..coords_no {
            let corner1 = path_vertices[x];
            let corner2 = path_vertices[y];
            inner = [
                [min(corner1[0], corner2[0]), min(corner1[1], corner2[1])],
                [max(corner1[0], corner2[0]), max(corner1[1], corner2[1])],
            ];

            if poly.intersects_bbox(inner, filled=True) and not poly.intersects_bbox(inner, filled=False):
                total = max(total, (abs(corner2[0] - corner1[0]) + 1) * (abs(corner2[1] - corner1[1]) + 1))

    return total
}
