#![allow(dead_code, unused_variables, unused_mut)]

use crate::solver::shared;

static DAY_NUM: u32 = 4;

pub fn task_a() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let len = file[0].len() + 2;
    let mut str = String::from("");

    for _ in 0..len {
        str.push('.');
    }

    for line in file {
        str.push('.');
        str.push_str(&line);
        str.push('.');
    }

    for _ in 0..len {
        str.push('.');
    }

    let chars = str.chars();

    for (pos, c) in chars.clone().enumerate() {
        if c != '@' {
            continue;
        }

        let mut count = 0;

        if chars.clone().nth(pos - len - 1).unwrap() == '@' {
            count += 1;
        }

        if chars.clone().nth(pos - len).unwrap() == '@' {
            count += 1;
        }

        if chars.clone().nth(pos - len + 1).unwrap() == '@' {
            count += 1;
        }

        if chars.clone().nth(pos - 1).unwrap() == '@' {
            count += 1;
        }

        if chars.clone().nth(pos + 1).unwrap() == '@' {
            count += 1;
        }

        if chars.clone().nth(pos + len - 1).unwrap() == '@' {
            count += 1;
        }

        if chars.clone().nth(pos + len).unwrap() == '@' {
            count += 1;
        }

        if chars.clone().nth(pos + len + 1).unwrap() == '@' {
            count += 1;
        }

        if count < 4 {
            result += 1;
        }
    }

    println!("Result for task A of day {}: {}", DAY_NUM, result); // RES: 1537
}

// Runs in 1/1000 compared to task_a()
pub fn task_a_2() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let len = file[0].len() + 2;
    let mut str = String::from("");

    for _ in 0..len {
        str.push('.');
    }

    for line in file {
        str.push('.');
        str.push_str(&line);
        str.push('.');
    }

    for _ in 0..len {
        str.push('.');
    }

    let mut chars: Vec<char> = str.chars().collect();

    for pos in 0..chars.len() {
        if chars[pos] != '@' {
            continue;
        }

        let mut count = 0;

        if chars[pos - len - 1] == '@' {
            count += 1;
        }

        if chars[pos - len] == '@' {
            count += 1;
        }

        if chars[pos - len + 1] == '@' {
            count += 1;
        }

        if chars[pos - 1] == '@' {
            count += 1;
        }

        if chars[pos + 1] == '@' {
            count += 1;
        }

        if chars[pos + len - 1] == '@' {
            count += 1;
        }

        if chars[pos + len] == '@' {
            count += 1;
        }

        if chars[pos + len + 1] == '@' {
            count += 1;
        }

        if count < 4 {
            result += 1;
        }
    }

    println!("Result for task A_2 of day {}: {}", DAY_NUM, result); // RES: 1537
}

pub fn task_b() {
    let file = shared::read_input(DAY_NUM, false);
    let mut result: u128 = 0;

    let len = file[0].len() + 2;
    let mut str = String::from("");

    for _ in 0..len {
        str.push('.');
    }

    for line in file {
        str.push('.');
        str.push_str(&line);
        str.push('.');
    }

    for _ in 0..len {
        str.push('.');
    }

    let mut chars: Vec<char> = str.chars().collect();

    let mut partial = 1;

    while partial > 0 {
        partial = 0;

        for pos in 0..chars.len() {
            if chars[pos] != '@' {
                continue;
            }

            let mut count = 0;

            if chars[pos - len - 1] == '@' {
                count += 1;
            }

            if chars[pos - len] == '@' {
                count += 1;
            }

            if chars[pos - len + 1] == '@' {
                count += 1;
            }

            if chars[pos - 1] == '@' {
                count += 1;
            }

            if chars[pos + 1] == '@' {
                count += 1;
            }

            if chars[pos + len - 1] == '@' {
                count += 1;
            }

            if chars[pos + len] == '@' {
                count += 1;
            }

            if chars[pos + len + 1] == '@' {
                count += 1;
            }

            if count < 4 {
                partial += 1;

                chars[pos] = '.';
            }
        }

        result += partial;
    }

    println!("Result for task B of day {}: {}", DAY_NUM, result); // RES: 8707
}
