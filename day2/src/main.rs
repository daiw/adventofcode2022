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
    points_for_shape(line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap())
        + points_for_result(line.chars().nth(2).unwrap())
}

fn points_for_shape(shape: char, my_goal: char) -> i32 {
    if shape == 'A' && my_goal == 'Y'
        || shape == 'B' && my_goal == 'X'
        || shape == 'C' && my_goal == 'Z'
    {
        return 1;
    }
    if shape == 'B' && my_goal == 'Y'
        || shape == 'C' && my_goal == 'X'
        || shape == 'A' && my_goal == 'Z'
    {
        return 2;
    }

    if shape == 'C' && my_goal == 'Y'
        || shape == 'A' && my_goal == 'X'
        || shape == 'B' && my_goal == 'Z'
    {
        return 3;
    }
    0
}

fn points_for_result(my_goal: char) -> i32 {
    if my_goal == 'X' {
        return 0;
    }
    if my_goal == 'Y' {
        return 3;
    }
    if my_goal == 'Z' {
        return 6;
    }
    0
}
