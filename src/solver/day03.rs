#![allow(dead_code, unused_variables, unused_mut)]

use crate::solver::shared;

static DAY_NUM: u32 = 3;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;
    
    for mut line in file {
        result += sub_task_a(line);
    }
    
    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 16812
}

fn sub_task_a(line: String) -> u128 {
    for d in (0..=9).rev() {
        match line.find(&d.to_string()) {
            Some(i_d) => {
    
                if i_d == line.len() - 1 {
                    continue;
                }
                
                let (_, last) = line.split_at(i_d + 1);

                for u in (0..=9).rev() {
                    match last.find(&u.to_string()) {
                        Some(i_u) => {
                            return d * 10 + u;
                        },
                        None => continue,
                    };
                }
            },
            None => continue,
        };
    }

    return 0;
}

pub fn task_a_2() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    for line in file {
        let mut sub_line: String;
        let mut pos=  0;
        let mut val: u128;
        let mut partial: u128 = 0;
        for i in 0..2 {
            let take = line.len() - pos - (2 - i) + 1;
            sub_line = line.chars().skip(pos).take(take).collect();
            
            let mut sub_pos;
            (val, sub_pos) = sub_task_b(sub_line, i);
                     
            pos += sub_pos + 1;
            partial *= 10;
            partial += val;
        }

        result += partial;
    }

    println!("Result for task A v.2 of day {}: {}", DAY_NUM, result); // RES: 16812
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    for line in file {
        let mut sub_line: String;
        let mut pos: usize=  0;
        let mut val: u128;
        let mut partial: u128 = 0;
        for i in 0..12 {
            sub_line = line.chars().skip(pos).take(line.len() - pos - (12 - i) + 1).collect();
            
            let mut sub_pos: usize;
            (val, sub_pos) = sub_task_b(sub_line, i);
                     
            pos += sub_pos + 1;
            partial *= 10;
            partial += val;
        }

        result += partial;
    }

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 166345822896410
}

fn sub_task_b(line: String, i: usize) -> (u128, usize) {
    for v in (0..=9).rev() {
        match line.find(&v.to_string()) {
            Some(i_v) => {
                return (v, i_v);
            },
            None => continue,
        };
    }

    return (0,0);
}