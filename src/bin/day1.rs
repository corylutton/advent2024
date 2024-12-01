use std::fs;


/// Advent of code day 1
fn main() {
    let (a_side, b_side) = parse_data();
    part_1(a_side.clone(), b_side.clone());
    part_2(a_side, b_side);
}

/// parse_data to get our data
///
/// Produces our two lists of numbers
fn parse_data() -> (Vec<i64>, Vec<i64>) {
    let mut a_side: Vec<i64> = Vec::new();
    let mut b_side: Vec<i64> = Vec::new();
    for line in fs::read_to_string("data/day1.txt").unwrap().lines() {
        if let Some((a, b)) = line.split_once("   ") {
            a_side.push(a.parse::<i64>().unwrap());
            b_side.push(b.parse::<i64>().unwrap());
        } else{
            println!("something went wrong");
        };
    }
    return (a_side, b_side);
}

/// part_1 of the days problems
///
/// `a_side` - The left side from the file
/// `b_side` - The right side from the file
fn part_1(mut a_side: Vec<i64>, mut b_side: Vec<i64>){
    a_side.sort();
    b_side.sort();
    let mut total = 0;
    for (i, a_val) in a_side.iter().enumerate(){
        let b_val = &b_side[i];
        if a_val <= b_val{
            total += b_val - a_val;
        } else {
            total += a_val - b_val;
        }
    }
    println!("Day1 - Part1 (sum smallest): {total}");
}

/// part_2 of the days problems
///
/// `a_side` - The left side from the file
/// `b_side` - The right side from the file
fn part_2(a_side: Vec<i64>, b_side: Vec<i64>){
    let mut similarity = 0;
    for a_val in a_side.iter(){
        let mut count = 0;

        for b_val in b_side.iter(){
            if b_val == a_val{
                count += 1;
            }
        }
        similarity += a_val * count;

    }
    println!("Day1 - Part2 (similarity): {similarity}");
}
