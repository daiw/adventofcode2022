use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "/workspaces/adventofcode/day4/input";

    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let x = reader
        .lines()
        .flatten()
        .filter(|line| is_contained(line))
        .count();

    println!("Count:\n{x}");
}

fn is_contained(line: &str) -> bool {
    let (comma_index, _) = line
        .chars()
        .enumerate()
        .find(|(_, c)| ','.eq(c))
        .expect("no comma found");
    let first = &line[0..comma_index];
    let second = &line[comma_index + 1..];

    let (first_minus, _) = first
        .chars()
        .enumerate()
        .find(|(_, c)| '-'.eq(c))
        .expect("1: no - found");
    let (second_minus, _) = second
        .chars()
        .enumerate()
        .find(|(_, c)| '-'.eq(c))
        .expect("2: no - found");

    let x1: i32 = first[..first_minus].parse().expect("no number");
    let x2: i32 = first[first_minus + 1..].parse().expect("no number");

    let y1: i32 = second[..second_minus].parse().expect("no number");
    let y2: i32 = second[second_minus + 1..].parse().expect("no number");

    (x1 >= y1 && x2 <= y2) || (y1 >= x1 && y2 <= x2)
}
