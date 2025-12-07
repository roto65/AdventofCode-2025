#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::{HashMap, HashSet};

use crate::solver::shared;

static DAY_NUM: u32 = 7;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut rays: HashSet<usize> = HashSet::new();

    rays.insert(file[0].find('S').unwrap());

    for line in file.iter().skip(1) {
        for (i, c) in line.chars().enumerate() {
            if c == '^' && rays.contains(&i) {
                rays.remove(&i);
                rays.insert(i + 1);
                rays.insert(i - 1);

                result += 1;
            }
        }
    }

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 1672
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;
    
    let start = file[0].find('S').unwrap();
    
    let mut cache: HashMap<(usize, usize), u128> = HashMap::new();
    result += sub_task_b(&file, 1, start, &mut cache);

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 231229866702355
}


fn sub_task_b(file: &Vec<String>, line_i: usize, ray_pos: usize, mut cache: &mut HashMap<(usize, usize), u128>) -> u128 {
    match cache.get(&(line_i, ray_pos)) {
        Some(val) => return *val,
        None => {},
    }   
    
    let mut ret: u128 = 0;

    for y in line_i..file.len() {
        if file[y].chars().nth(ray_pos).unwrap() == '^' {
            ret += sub_task_b(file, y + 1, ray_pos - 1, cache);
            ret += sub_task_b(file, y + 1, ray_pos + 1, cache);

            cache.insert((line_i, ray_pos), ret);

            return ret;
        }
    }

    return 1;
}
