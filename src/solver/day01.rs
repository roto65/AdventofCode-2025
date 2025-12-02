#![allow(dead_code, unused_variables, unused_mut)]

use crate::solver::shared;

static DAY_NUM: u32 = 1;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut state: i32 = 50;

    for line in file {
        let (dir, amount) = line.split_at(1);

        if dir == "R" {
            state += amount.trim().parse::<i32>().unwrap();
        } else {
            state -= amount.trim().parse::<i32>().unwrap();
        }

        if state % 100 == 0 {
            result += 1;
        }
    }

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 969
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut state: i32 = 50;

    for line in file {
        let (dir, amount) = line.split_at(1);

        let mut delta = amount.trim().parse::<i32>().unwrap();

        result += (delta / 100) as u128;
        delta = delta % 100;

        let increment = if dir == "R" { 1 } else { -1 };

        for _ in 0..delta {
            state += increment;

            if state % 100 == 0 {
                result += 1;
            }
        }
    }

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 5887
}
