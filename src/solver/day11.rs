#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;

use crate::solver::shared;

static DAY_NUM: u32 = 11;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cache: HashMap<String, u128> = HashMap::new();

    for line in file {
        let (key, value) = line.split_at(line.find(':').unwrap());

        map.insert(
            key.to_string(),
            value
                .strip_prefix(':')
                .unwrap()
                .trim()
                .split(' ')
                .map(String::from)
                .collect(),
        );
    }

    result += dfs_a(String::from("you"), &map, &mut cache);

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 603
}

fn dfs_a(
    curr: String,
    map: &HashMap<String, Vec<String>>,
    mut cache: &mut HashMap<String, u128>,
) -> u128 {
    let mut ret = 0;

    if cache.contains_key(&curr) {
        return *cache.get(&curr).unwrap();
    }

    if curr == String::from("out") {
        return 1;
    }

    for val in map.get(&curr).unwrap() {
        let mut tmp = dfs_a(val.clone(), map, cache);

        cache.insert(val.clone(), tmp);
        ret += tmp;
    }

    return ret;
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut cache: HashMap<(String, bool, bool), u128> = HashMap::new();

    for line in file {
        let (key, value) = line.split_at(line.find(':').unwrap());

        map.insert(
            key.to_string(),
            value
                .strip_prefix(':')
                .unwrap()
                .trim()
                .split(' ')
                .map(String::from)
                .collect(),
        );
    }

    result += dfs_b(String::from("svr"), &map, &mut cache, false, false);

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 380961604031372
}

fn dfs_b(
    curr: String,
    map: &HashMap<String, Vec<String>>,
    mut cache: &mut HashMap<(String, bool, bool), u128>,
    flg_dac: bool,
    flg_fft: bool,
) -> u128 {
    let mut ret = 0;

    if cache.contains_key(&(curr.clone(), flg_dac, flg_fft)) {
        return *cache.get(&(curr, flg_dac, flg_fft)).unwrap();
    }

    if curr == String::from("out") {
        if flg_dac && flg_fft {
            return 1;
        } else {
            return 0;
        }
    }

    for val in map.get(&curr).unwrap() {
        let tmp_flg_dac = flg_dac || val == &String::from("dac");
        let tmp_flg_fft = flg_fft || val == &String::from("fft");

        let tmp = dfs_b(val.clone(), map, cache, tmp_flg_dac, tmp_flg_fft);

        cache.insert((val.clone(), tmp_flg_dac, tmp_flg_fft), tmp);
        ret += tmp;
    }

    return ret;
}
