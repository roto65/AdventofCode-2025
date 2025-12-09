#![allow(dead_code, unused_variables, unused_mut)]

use ac_library::Dsu;

use crate::solver::shared;

static DAY_NUM: u32 = 8;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut nodes: Vec<(i128, i128, i128)> = vec![];

    let file_len = file.len();

    for line in file {
        let v: Vec<&str> = line.split(",").collect();
        nodes.push((
            v[0].trim().parse().unwrap(),
            v[1].trim().parse().unwrap(),
            v[2].trim().parse().unwrap(),
        ));
    }

    let mut dists: Vec<((usize, usize), f64)> = vec![];

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let dist = (((nodes[i].0 - nodes[j].0).abs().pow(2)
                + (nodes[i].1 - nodes[j].1).abs().pow(2)
                + (nodes[i].2 - nodes[j].2).abs().pow(2)) as f64)
                .sqrt();

            dists.push(((i, j), dist));
        }
    }

    dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut dsu = Dsu::new(file_len);

    for &((u, v), _) in dists.iter().take(1000) {
        dsu.merge(u as usize, v as usize);
    }

    let mut components: Vec<usize> = dsu.groups().iter().map(|v| v.len()).collect();
    components.sort();
    components.reverse();

    result += components
        .iter()
        .take(3)
        .map(|elem| *elem as u128)
        .product::<u128>();

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 67488
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let mut nodes: Vec<(i128, i128, i128)> = vec![];

    let file_len = file.len();

    for line in file {
        let v: Vec<&str> = line.split(",").collect();
        nodes.push((
            v[0].trim().parse().unwrap(),
            v[1].trim().parse().unwrap(),
            v[2].trim().parse().unwrap(),
        ));
    }

    let mut dists: Vec<((usize, usize), f64)> = vec![];

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let dist = (((nodes[i].0 - nodes[j].0).abs().pow(2)
                + (nodes[i].1 - nodes[j].1).abs().pow(2)
                + (nodes[i].2 - nodes[j].2).abs().pow(2)) as f64)
                .sqrt();

            dists.push(((i, j), dist));
        }
    }

    dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    let mut dsu = Dsu::new(file_len);

    for ((u, v), _) in dists {
        dsu.merge(u as usize, v as usize);

        if dsu.groups().len() == 1 {
            result += (nodes[u].0 * nodes[v].0) as u128;
            break;
        }
    }

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 3767453340
}
