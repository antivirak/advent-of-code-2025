#![allow(clippy::unused_unit)]

use std::collections::BinaryHeap;
use crate::*;

// https://adventofcode.com/2025/day/8
// 08:09:13 / 13:02:26


pub fn main_1() -> usize {
    let coords = {
        open("day08/input.txt", "r").readlines()
        .iter()
        .map(|row| row.split(",").map(int).collect::<Vec<usize>>())
        .collect::<Vec<Vec<_>>>()
    };
    let coords_no = len(&coords);

    let mut map = dict();
    for x in 0..coords_no - 1 {
        for y in x + 1..coords_no {
            // No need to sqrt for actual distance
            map.insert((x, y), //f32::sqrt(
                np::substract(coords[x].clone(), coords[y].clone()).iter().map(|x| x * x).sum::<usize>()  // as f32
            //)
            );
        }
    }

    const SIZE: usize = 1000;  // 10 for test
    let adjacency = map.items()
        //.sorted_by(|a, b| f32::total_cmp(a.1, b.1))
        .sorted_by_key(|a| a.1)
        .take(SIZE)
        .map(|(key, _)| *key)
        .collect::<Vec<(usize, usize)>>();

    // find connected components using BFS
    let mut marks = [0; 1000];  // this can stay 1000 for test
    let mut components = 0;
    let mut comp_to_vs = dict();
    let mut q = vec![];  // stack; in theory it should be queue, but stack works here
    for (count, vertex) in enumerate(&adjacency) {
        if marks[count] != 0 {
            continue;
        }
        components += 1;
        comp_to_vs.insert(components, set());
        q.push((count, vertex));
        while let Some((vs_idx, vs)) = q.pop() {
            marks[vs_idx] = components;
            comp_to_vs.entry(components).or_default().update([vs.0, vs.1]);
            enumerate(&adjacency)
                .filter(|(_, item)| item.0 == vs.0 || item.1 == vs.0 || item.0 == vs.1 || item.1 == vs.1)
                .for_each(|(c, item)| {
                    if marks[c] == 0 {
                        q.push((c, item));
                    }
                });
        }
    }

    comp_to_vs.values()
        .map(|val| len(&val))
        .sorted().rev().take(3).product()
}


pub fn main_2() -> usize {
    let coords = {
        open("day08/input.txt", "r").readlines()
        .iter()
        .map(|row| row.split(",").map(int).collect::<Vec<usize>>())
        .collect::<Vec<Vec<_>>>()
    };
    let coords_no = len(&coords);

    let mut map = dict();
    for x in 0..coords_no - 1 {
        for y in x + 1..coords_no {
            map.insert((x, y),
                np::substract(coords[x].clone(), coords[y].clone()).iter().map(|x| x * x).sum::<usize>()
            );
        }
    }

    const SIZE: usize = 5000;  // 4500 is 2 clusters already
    let adjacency = map.items()
        .sorted_by_key(|a| a.1)
        .take(SIZE)
        .map(|(key, _)| *key)
        .collect::<Vec<(usize, usize)>>();

    let mut vs_to_adj = dict();
    for idx in 0..coords_no {
        vs_to_adj.insert(idx, enumerate(&adjacency)
            .filter(|(_, item)| item.0 == idx || item.1 == idx)
            .collect::<Vec<_>>());
    }

    let mut visited_nodes = set();
    let mut visited_edges = set();
    let mut q = BinaryHeap::new();  // priority based on location in adjacency
    let vertex = adjacency[0];
    q.push((0, vertex));
    // vs should actually be es
    while let Some((_, vs)) = q.pop() {
        let len_vis_prev = len(&visited_nodes);
        visited_nodes.add(vs.0);
        visited_nodes.add(vs.1);
        let len_vis = len(&visited_nodes);
        if len_vis == len_vis_prev {
            continue;
        }
        visited_edges.add(vs);
        if len_vis >= coords_no {
            return coords[vs.0][0] * coords[vs.1][0];
        }
        vs_to_adj[&vs.0].iter().chain(vs_to_adj[&vs.1].iter())
            .for_each(|(c, item)| {
                if !visited_edges.contains(item) {
                    q.push((SIZE - c, *item));  // making lowest-first from greatest-first heapq
                }
            });
    }

    unreachable!();
}
