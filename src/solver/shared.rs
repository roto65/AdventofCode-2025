use std::{fs, time::Instant};

pub fn read_input(num: u32, test: bool) -> Vec<String> {
    let content = if test {
        fs::read_to_string(format!("data/day{:02}.test.txt", num)).unwrap()
    } else {
        fs::read_to_string(format!("data/day{:02}.txt", num)).unwrap()
    };

    return content.split("\n").into_iter().map(str::trim).map(String::from).collect();
}

pub fn timer<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();

    println!("Execution time: {:?}", elapsed);
    return result;
}
