#![allow(dead_code, unused_variables, unused_mut)]

use fancy_regex::Regex;

use crate::solver::shared;

static DAY_NUM: u32 = 2;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let intervals: Vec<String> = file[0].split(",").into_iter().map(String::from).collect();

    for interval in intervals {
        let (first, last): (u128, u128) = interval
            .split_once('-')
            .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
            .unwrap();

        for n in first..=last {
            let n_str = n.to_string();
            let (a, b) = n_str.split_at(n_str.len() / 2);

            if a == b {
                result += n;
            }
        }
    }

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 17077011375
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let re = Regex::new("^(\\d+)\\1+$").unwrap();
    let intervals: Vec<String> = file[0].split(",").into_iter().map(String::from).collect();

    for interval in intervals {
        let (first, last): (u128, u128) = interval
            .split_once('-')
            .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
            .unwrap();

        for n in first..=last {
            let n_str = n.to_string();

            if re.is_match(&n_str).unwrap() {
                result += n;
            }
        }
    }

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 36037497037
}
