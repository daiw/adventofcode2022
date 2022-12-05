use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file_path = "/workspaces/adventofcode/day3/input";

    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    let mut prio_sum = 0;

    reader.lines().for_each(|line| {
        if let Ok(l) = line {
            prio_sum += compute_prio(l);
        }
    });

    // println!("Test {:?}: {:?}", 'a', get_prio('a'));
    // println!("Test {:?}: {:?}", 'z', get_prio('z'));
    // println!("Test {:?}: {:?}", 'A', get_prio('A'));
    // println!("Test {:?}: {:?}", 'Z', get_prio('Z'));

    println!("Prio:\n{prio_sum}");
}

fn compute_prio(line: String) -> i32 {
    let item = find_double_item(&line[0..line.len() / 2], &line[line.len() / 2..line.len()]);

    if let Some(i) = item {
        return get_prio(i);
    }

    0
}

fn find_double_item(items_1: &str, items_2: &str) -> Option<char> {
    items_1.chars().find(|c| items_2.chars().any(|c2| c2.eq(c)))
}

fn get_prio(item: char) -> i32 {
    if item as i32 >= 'a' as i32 {
        return item as i32 - 'a' as i32 + 1;
    }

    item as i32 - 'A' as i32 + 27
}
