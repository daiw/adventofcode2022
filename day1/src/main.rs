use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "/workspaces/adventofcode/day1/input";

    // let contents = fs::read read_to_string("/workspaces/adventofcode/day1/input")
    //     .expect("Should have been able to read the file");
    // println!("With text:\n{contents}");

    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    let mut max = [0, 0, 0];

    for line in reader.lines().flatten() {
        if !line.is_empty() {
            let number = line.parse::<i32>().unwrap();
            sum += number;
        } else if sum > max[0] {
            max[0] = sum;
            sum = 0;
            max.sort();
            println!("Sorted: ");
            for b in max {
                print!("{b} ");
            }
            println!("\n");
        } else {
            sum = 0;
        }
    }

    sum=0;
    for b in max {
        sum+=b;
    }

    println!("sum:\n{sum}");
}
