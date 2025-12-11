#![allow(clippy::unused_unit)]

//use collections::VecDeque;
use crate::*;

// https://adventofcode.com/2025/day/10
// 


pub fn main_1() -> usize {
    let binding = {
        open("day10/input.txt", "r").readlines()
    };
    let input = binding.iter()
        //.map(|row| row.splitn(1, " "))
        .map(|row| row.split_once(" ").unwrap())
        .map(|pair| {
            let tail: (_, _) = pair.1.rsplitn(2, " ").collect_tuple().unwrap();  // ::<Vec<_>>()
            vec![pair.0, tail.0, tail.1]
        })
        .collect::<Vec<Vec<_>>>();
    let transposed = (0..3)
        .map(|i| input.iter().map(|inner| inner[i]).collect()).collect::<Vec<Vec<_>>>();
    let lights = &transposed[0]
        .iter()
        .map(|x| {
            let chars = x.chars().collect::<Vec<_>>();
            chars[1..len(&chars) - 1]
            //.to_vec()
            .iter()
            .map(|y| *y == '#')
            .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();
    let btns_unsplit = transposed[2].clone().iter()
        .map(|row| row.split(" ")
            .map(|tuple| {
                let chars = tuple.chars().collect::<Vec<_>>();
                chars[1..len(&chars) - 1].iter().copied()
                    //.collect::<Vec<_>>().iter()
                    .collect::<String>()
                    // .split(",")  // cannot split here; would return values owned by this closure. Maybe Box would help
            }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    //let btns = btns_unsplit.into_iter()
    //    .map(|x| x.split(',').collect::<Vec<_>>())
    //    .collect::<Vec<Vec<_>>>();
    let size = len(&btns_unsplit);
    let mut btns_split = vec![vec![]; size];
    // could probably be solved by for_each()
    for (count, btns) in enumerate(&btns_unsplit) {
        for btn in btns {
            btns_split[count].push(btn.split(",")
                .map(int).collect::<Vec<usize>>());
        }
    }
    let mut total = 0;
    for (count, btns) in enumerate(&btns_split) {
        // each row is a different problem, we need to construct the paths separately
        // probably DP problem
        let target = &lights[count];
        //let mut currents = VecDeque::new()
        //currents.push(vec![false; target.len()]);
        let mut currents = vec![vec![false; target.len()]];
        let mut step = 0;
        // BFS; priority heap q based on string distance could help, but maybe not really
        while !currents.contains(target) {
            step += 1;
            let mut new = vec![];
            for current in currents {
                for btn in &btns {
                    // TODO do not apply the same next time - shift by one in every even run
                    let mut current_instance = current.clone();
                    for idx in btn {
                        current_instance[*idx] = !current_instance[*idx];
                    }
                    //print(&current_instance.iter().map(|x| {
                    //    if *x {
                    //        '#'
                    //    } else {
                    //        '.'
                    //    }
                    //}).collect::<Vec<_>>());
                    new.push(current_instance)
                }
            }
            currents = new;
        }
        print(&(count as f32 / size as f32));
        total += step;
    }

    total
}


pub fn main_2() -> usize {
    // works on test. Use the DP!
    let binding = {
        open("day10/test_input.txt", "r").readlines()
    };
    let input = binding.iter()
        //.map(|row| row.splitn(1, " "))
        .map(|row| row.split_once(" ").unwrap())
        .map(|pair| {
            let tail: (_, _) = pair.1.rsplitn(2, " ").collect_tuple().unwrap();  // ::<Vec<_>>()
            vec![pair.0, tail.0, tail.1]
        })
        .collect::<Vec<Vec<_>>>();
    let transposed = (0..3)
        .map(|i| input.iter().map(|inner| inner[i]).collect()).collect::<Vec<Vec<_>>>();

    let joltage = &transposed[1]
        .iter()
        .map(|x| {
            let chars = x.chars().collect::<Vec<_>>();
            chars[1..len(&chars) - 1]
            //.to_vec()
            .iter()
            .collect::<String>()
            .split(",")
            .map(|y| {
                //print(&y);
                int::<usize>(y)
            })
            .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<_>>>();
    let btns_unsplit = transposed[2].clone().iter()
        .map(|row| row.split(" ")
            .map(|tuple| {
                let chars = tuple.chars().collect::<Vec<_>>();
                chars[1..len(&chars) - 1].iter().copied()
                    //.collect::<Vec<_>>().iter()
                    .collect::<String>()
                    // .split(",")  // cannot split here; would return values owned by this closure. Maybe Box would help
            }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    //let btns = btns_unsplit.into_iter()
    //    .map(|x| x.split(',').collect::<Vec<_>>())
    //    .collect::<Vec<Vec<_>>>();
    let size = len(&btns_unsplit);
    let mut btns_split = vec![vec![]; size];
    // could probably be solved by for_each()
    for (count, btns) in enumerate(&btns_unsplit) {
        for btn in btns {
            btns_split[count].push(btn.split(",")
                .map(int).collect::<Vec<usize>>());
        }
    }
    let mut total = 0;
    for (count, btns) in enumerate(&btns_split) {
        // each row is different problem, we need to construct the paths separately
        // probably DP problem
        let target = &joltage[count];
        //let mut currents = VecDeque::new()
        //currents.push(vec![false; target.len()]);
        let mut currents = vec![vec![0; target.len()]];
        let mut step = 0;
        // BFS; priority heap q based on string distance could help, but maybe not really
        while !currents.contains(target) {
            step += 1;
            let mut new = vec![];
            for current in currents {
                for btn in &btns {
                    let mut current_instance = current.clone();
                    for idx in btn {
                        current_instance[*idx] += 1;
                    }
                    new.push(current_instance)
                }
            }
            currents = new;
        }
        print(&(count as f32 / size as f32));
        total += step;
    }

    total
}
