use std::{fs, ops::Index};

// Advent of code day 2
fn main() {
    report_one();
    report_two();
}

/// Report one
fn report_one() {
    let mut total = 0;
    let mut notsafe = 0;

    for line in fs::read_to_string("data/day2.txt").unwrap().lines() {
        let report: Vec<i64> = line
            .split(" ")
            .filter_map(|l| i64::from_str_radix(l, 10).ok())
            .collect();

        total += 1;
        let mut direction = 0;
        let mut failed = false;

        for (i, r) in report.iter().enumerate().skip(1) {
            let last = report.index(i - 1);
            let diff = last - r;

            // Check for too much amplitude or little
            if diff.abs() > 3 || diff.abs() < 1 {
                failed = true;
                break;
            }

            // Check if we change direction
            let current_direction = if diff > 0 {
                1
            } else if diff < 0 {
                -1
            } else {
                0
            };

            if direction == 0 {
                direction = current_direction;
            } else if direction != current_direction {
                failed = true;
                break;
            }
        }
        if failed {
            notsafe += 1;
        }
    }
    println!("Part1 - Reports that are safe: {}", total - notsafe);
}

/// Report two
fn report_two() {
    let mut total = 0;
    let mut notsafe = 0;

    for line in fs::read_to_string("data/day2.txt").unwrap().lines() {
        let report: Vec<i64> = line
            .split(" ")
            .filter_map(|l| i64::from_str_radix(l, 10).ok())
            .collect();

        total += 1;
        let mut direction = 0;
        let mut failed = 0;

        for (i, r) in report.iter().enumerate().skip(1) {
            let last = report.index(i - 1);
            let diff = last - r;

            // Check for too much amplitude or little
            if diff.abs() > 3 || diff.abs() < 1 {
                failed += 1;
                continue;
            }

            // Check if we change direction
            let current_direction = if diff > 0 {
                1
            } else if diff < 0 {
                -1
            } else {
                0
            };

            // What happens if our first direction is wrong?
            if direction == 0 {
                direction = current_direction;
            } else if direction != current_direction {
                failed += 1;
                continue;
            }
        }
        if failed > 2 {
            notsafe += 1;
            println!("{line}");
        }
    }
    println!("Part2 - Reports that are safe: {}", total - notsafe);
}
