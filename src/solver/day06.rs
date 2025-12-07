#![allow(dead_code, unused_variables, unused_mut)]

use crate::solver::shared;

static DAY_NUM: u32 = 6;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut vals: Vec<Vec<u32>> = vec![];

    for i in 0..file.len() {
        if i == file.len() - 1 {
            break;
        }

        vals.push(
            file[i]
                .split_whitespace()
                .map(|value| value.trim().parse().unwrap())
                .collect(),
        );
    }

    let ops: Vec<&str> = file[file.len() - 1].split_whitespace().collect();

    for i in 0..ops.len() {
        match ops[i] {
            "+" => {
                let mut partial = 0;
                for val_vec in vals.clone() {
                    partial += val_vec[i];
                }

                result += partial as u128;
            }
            "*" => {
                let mut partial: u128 = 1;
                for val_vec in vals.clone() {
                    partial *= val_vec[i] as u128;
                }

                result += partial;
            }
            _ => {}
        }
    }

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 6343365546996
}

pub fn task_b() {
    let mut file = shared::read_input_untrimmed(DAY_NUM, false);
    let mut result: u128 = 0;

    let ops_str = file.pop().unwrap();
    let ops: Vec<&str> = ops_str.split_whitespace().collect();

    let len = file[0].len();

    let mut lines: Vec<Vec<char>> = vec![];

    for line in file {
        lines.push(line.chars().collect());
    }

    let mut i_ops = ops.len() - 1;
    let mut val_vec: Vec<u128> = vec![];

    for i in (0..len - 1).rev() {
        let mut ss: String = String::from("");

        for line in &lines {
            ss.push(line[i]);
        }

        if ss.trim() == String::from("") || i == 0 {
            if i == 0 {
                val_vec.push(ss.trim().parse().unwrap());
            }

            match ops[i_ops] {
                "+" => {
                    result += val_vec.clone().iter().sum::<u128>();
                    val_vec = vec![];
                }
                "*" => {
                    result += val_vec.clone().iter().product::<u128>();
                    val_vec = vec![];
                }
                _ => {}
            };

            if i != 0 {
                i_ops -= 1;
            }
        } else {
            val_vec.push(ss.trim().parse().unwrap());
        }
    }

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 11136895955912
}
