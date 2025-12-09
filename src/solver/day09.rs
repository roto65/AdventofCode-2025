#![allow(dead_code, unused_variables, unused_mut)]

use itertools::Itertools;

use crate::solver::shared;

static DAY_NUM: u32 = 9;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut tiles: Vec<(i128, i128)> = vec![];

    for line in file {
        let v: Vec<&str> = line.split(",").collect();
        tiles.push((v[0].trim().parse().unwrap(), v[1].trim().parse().unwrap()));
    }

    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let area =
                ((tiles[i].0 - tiles[j].0 + 1).abs() * (tiles[i].1 - tiles[j].1 + 1).abs()) as u128;
            result = result.max(area);
        }
    }

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 4725826296
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut tiles: Vec<(u64, u64)> = vec![];

    for line in file {
        let v: Vec<&str> = line.split(",").collect();
        tiles.push((v[0].trim().parse().unwrap(), v[1].trim().parse().unwrap()));
    }

    result += tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (area(a, b), (a, b)))
        .sorted_by_key(|(area, _)| std::cmp::Reverse(*area))
        .find(|(_, rect)| {
            !tiles
                .iter()
                .chain(std::iter::once(&tiles[0]))
                .tuple_windows()
                .any(|line| is_intersecting(line, *rect))
        })
        .unwrap()
        .0;

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 1637556834
}

fn area(a: &(u64, u64), b: &(u64, u64)) -> u128 {
    return ((a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)) as u128;
}

fn bounds((a, b): (&(u64, u64), &(u64, u64))) -> ((u64, u64), (u64, u64)) {
    return ((a.0.min(b.0), a.1.min(b.1)), (a.0.max(b.0), a.1.max(b.1)));
}

fn is_intersecting(line: (&(u64, u64), &(u64, u64)), rect: (&(u64, u64), &(u64, u64))) -> bool {
    let (lmin, lmax) = bounds(line);
    let (rmin, rmax) = bounds(rect);

    return line.0 .0 == line.1 .0
        && (((lmin.1 < rmin.1 && lmax.1 > rmin.1)
            || (lmin.1 < rmax.1 && lmax.1 > rmax.1)
            || (lmin.1 >= rmin.1 && lmax.1 <= rmax.1))
            && line.0 .0 > rmin.0
            && line.0 .0 < rmax.0)
        || line.0 .1 == line.1 .1
            && (((lmin.0 < rmin.0 && lmax.0 > rmin.0)
                || (lmin.0 < rmax.0 && lmax.0 > rmax.0)
                || (lmin.0 >= rmin.0 && lmax.0 <= rmax.0))
                && line.1 .1 > rmin.1
                && line.1 .1 < rmax.1);
}
