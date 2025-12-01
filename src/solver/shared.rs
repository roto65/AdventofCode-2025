use std::fs;

pub fn read_input(num: u32, test: bool) -> Vec<String> {
    let content = if test {
        fs::read_to_string(format!("data/day{}.test.txt", num)).unwrap()
    } else {
        fs::read_to_string(format!("data/day{}.txt", num)).unwrap()
    };

    return content.split("\n").into_iter().map(String::from).collect();
}