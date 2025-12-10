#![allow(dead_code, unused_variables, unused_mut)]

use std::{
    collections::{HashMap, VecDeque},
    vec,
};

use crate::solver::shared;

static DAY_NUM: u32 = 10;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    for line in file {
        let elems: Vec<&str> = line.split_whitespace().collect();

        let mut state_v: Vec<u8> = vec![];

        for light in elems[0].chars() {
            match light {
                '.' => state_v.push(0),
                '#' => state_v.push(1),
                _ => {}
            }
        }

        let mut state: u32 = state_to_u32(&state_v);

        let mut btns: Vec<u32> = vec![];

        for mut elem in elems.iter().skip(1).take_while(|e| e.starts_with('(')) {
            let elem = &elem.strip_prefix('(').unwrap();
            let elem = &elem.strip_suffix(')').unwrap();

            let mut btn: Vec<u8> = vec![0; state_v.len()];

            for n in elem.split(',').map(|n| n.parse::<usize>().unwrap()) {
                btn[n] = 1;
            }

            btns.push(state_to_u32(&btn));
        }

        result += bfs_a(state, &btns);
    }

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 396
}

fn state_to_u32(state: &Vec<u8>) -> u32 {
    let mut val: u32 = 0;

    for bit in state {
        val = val << 1;
        val = val ^ *bit as u32;
    }

    return val;
}

fn bfs_a(initial_state: u32, btns: &Vec<u32>) -> u128 {
    let mut queue: VecDeque<u32> = VecDeque::from([initial_state]);
    let mut visited: HashMap<u32, u32> = HashMap::from([(initial_state, u32::MAX)]);

    while let Some(current_state) = queue.pop_front() {
        if current_state == 0 {
            return backtrack_a(&current_state, &visited).len() as u128;
        }

        for btn in btns {
            let next_state = current_state ^ *btn;

            if !visited.contains_key(&next_state) {
                visited.insert(next_state, current_state);
                queue.push_back(next_state);
            }
        }
    }

    return 0;
}

fn backtrack_a(goal_state: &u32, visited: &HashMap<u32, u32>) -> Vec<u32> {
    let mut path: Vec<u32> = vec![];
    let mut current_state = goal_state;

    while let Some(parent) = visited.get(&current_state) {
        if parent == &u32::MAX {
            break;
        }

        path.push(*parent);
        current_state = parent;

        if path.len() > 100 {
            break;
        }
    }

    path.reverse();
    return path;
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, true);
    let mut result: u128 = 0;

    for line in file {
        let elems: Vec<&str> = line.split_whitespace().skip(1).collect();

        let target: Vec<u8> = elems
            .last()
            .unwrap()
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let mut btns: Vec<Vec<u8>> = vec![];

        for mut elem in elems.iter().take_while(|e| e.starts_with('(')) {
            let elem = &elem.strip_prefix('(').unwrap();
            let elem = &elem.strip_suffix(')').unwrap();

            let mut btn: Vec<u8> = vec![0; target.len()];

            for n in elem.split(',').map(|n| n.parse::<usize>().unwrap()) {
                btn[n] = 1;
            }

            btns.push(btn);
        }

        result += bfs_b(&target, &btns);
    }

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 15688
}

fn bfs_b(target_state: &Vec<u8>, btns: &Vec<Vec<u8>>) -> u128 {
    let mut queue: VecDeque<Vec<u8>> = VecDeque::from([vec![0u8; target_state.len()]]);
    let mut visited: HashMap<Vec<u8>, Vec<u8>> = HashMap::from([(
        vec![0u8; target_state.len()],
        vec![u8::MAX; target_state.len()],
    )]);

    while let Some(current_state) = queue.pop_front() {
        if current_state == *target_state {
            return backtrack_b(&current_state, &visited).len() as u128;
        }

        for btn in btns {
            let next_state: Vec<u8> = current_state.iter().zip(btn).map(|(s, b)| s + b).collect();

            if next_state.iter().zip(target_state).any(|(n, t)| n > t) {
                continue;
            }

            if !visited.contains_key(&next_state.clone()) {
                visited.insert(next_state.clone(), current_state.clone());
                queue.push_back(next_state);
            }
        }
    }

    return 0;
}

fn backtrack_b(last_state: &Vec<u8>, visited: &HashMap<Vec<u8>, Vec<u8>>) -> Vec<Vec<u8>> {
    let mut path: Vec<Vec<u8>> = vec![];
    let mut current_state = last_state;

    while let Some(parent) = visited.get(&current_state.clone()) {
        if parent == &vec![u8::MAX; last_state.len()] {
            break;
        }

        path.push(parent.clone());
        current_state = parent;

        if path.len() > 100 {
            break;
        }
    }

    path.reverse();
    return path;
}
