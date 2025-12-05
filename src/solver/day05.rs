#![allow(dead_code, unused_variables, unused_mut)]

use crate::solver::shared;

static DAY_NUM: u32 = 5;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut split: usize = 0;

    let mut intervals: Vec<(u64, u64)> = vec![];

    for i in 0..file.len() {
        if file[i] == String::from("") {
            split = i + 1;
            break;
        }

        file[i]
            .split_once('-')
            .map(|(a, b)| (intervals.push((a.trim().parse().unwrap(), b.trim().parse().unwrap()))));
    }

    for i in split..file.len() {
        let num: u64 = file[i].trim().parse().unwrap();

        for elem in intervals.clone() {
            if num >= elem.0 && num <= elem.1 {
                result += 1;
                break;
            }
        }
    }

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 615
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut intervals: Vec<(u64, u64)> = vec![];

    for i in 0..file.len() {
        if file[i] == String::from("") {
            break;
        }

        file[i]
            .split_once('-')
            .map(|(a, b)| (intervals.push((a.trim().parse().unwrap(), b.trim().parse().unwrap()))));
    }

    intervals.sort();

    let mut merged = Vec::new();
    let mut current = intervals[0];

    for &(start, end) in intervals.iter().skip(1) {
        if start <= current.1 {
            current.1 = current.1.max(end);
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    result += merged
        .iter()
        .map(|&(s, e)| e - s + 1)
        .sum::<u64>() as u128;

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 353716783056994
}
