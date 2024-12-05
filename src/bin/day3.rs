use regex::Regex;
use std::fs;

// Advent of code day 2
fn main() {
    let data = load_data();
    report_one(&data);
}

fn load_data() -> String {
    return fs::read_to_string("data/day3.txt").unwrap();
}

/// Report one
fn report_one(data: &String) {
    let mut total = 0;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    for (_, [num1, num2]) in re.captures_iter(data).map(|c| c.extract()) {
        total += i64::from_str_radix(num1, 10).unwrap() * i64::from_str_radix(num2, 10).unwrap();
    }
    println!("Part1 - Total: {}", total);
}
