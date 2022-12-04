use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "/workspaces/adventofcode/day2/input";

    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut points = 0;

    for line in reader.lines().flatten() {
        if !line.is_empty() {
            points += compute_points(line);
        }
    }

    println!("Points:\n{points}");
}

fn compute_points(line: String) -> i32 {
    points_for_shape(line.chars().nth(2).unwrap())
        + points_for_result(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap())
}

fn points_for_shape(shape: char) -> i32 {
    if shape == 'X' {
        return 1;
    }
    if shape == 'Y' {
        return 2;
    }
    if shape == 'Z' {
        return 3;
    }
    0
}

fn points_for_result(other_shape: char, my_shape: char) -> i32 {
    if other_shape == 'A' && my_shape == 'X'
        || other_shape == 'B' && my_shape == 'Y'
        || other_shape == 'C' && my_shape == 'Z'
    {
        return 3;
    }

    if other_shape == 'C' && my_shape == 'X'
        || other_shape == 'A' && my_shape == 'Y'
        || other_shape == 'B' && my_shape == 'Z'
    {
        return 6;
    }

    0
}
