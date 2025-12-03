#![allow(clippy::unused_unit)]

use crate::*;

// https://adventofcode.com/2025/day/3
// 00:40:32 / 02:31:24


pub fn main_1() -> i32 {
    let banks = {
        open("day03/input.txt", "r").readlines()
    };
    let batteries_in_banks = banks[0].chars().count();
    let mut total = 0;
    for bank in banks {
        // TODO let int fnc deref &str from char
        let batteries = map(|x| int::<i32>(&x.to_string()), bank.chars()).collect::<Vec<_>>();
        let mut max_val = batteries.iter().max().unwrap();
        let mut m_val_id = list::index(&batteries, *max_val);
        if m_val_id == batteries_in_banks - 1 {
            max_val = batteries[..(batteries_in_banks as usize) - 1].iter().max().unwrap();
            m_val_id = list::index(&batteries, *max_val);
        }
        let second_max = batteries[m_val_id + 1..].iter().max().unwrap();
        total += max_val * 10 + second_max;
    }

    total
}


pub fn main_2() -> i64 {
    let banks = {
        open("day03/input.txt", "r").readlines()
    };
    let batteries_in_banks_init = banks[0].chars().count();
    let mut total = 0;
    for bank in banks {
        let mut batteries_in_banks = batteries_in_banks_init;
        let mut batteries = map(|x| int::<i32>(&x.to_string()), bank.chars()).collect::<Vec<_>>();
        let mut max_val;
        let mut m_val_id;
        for mut threshold in 0..12 {
            threshold = 11 - threshold;
            max_val = batteries.iter().max().unwrap();
            m_val_id = list::index(&batteries, *max_val);

            let difference = batteries_in_banks - threshold;
            while m_val_id >= difference.try_into().unwrap() {
                max_val = batteries[..(difference as usize)].iter().max().unwrap();
                m_val_id = list::index(&batteries, *max_val);
            }

            total += *max_val as i64 * pow(10, threshold);
            batteries = batteries[m_val_id + 1..].to_vec();
            batteries_in_banks = batteries.len();
        }
    }

    total
}
