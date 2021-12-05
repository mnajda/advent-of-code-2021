use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Coords {
    x: u64,
    y: u64,
}

fn make_coords(values: &(u64, u64)) -> Coords {
    Coords {
        x: values.0,
        y: values.1
    }
}

fn load_file(path: &String) -> Vec<(Coords, Coords)> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    let values: Vec<(u64, u64)> = contents
        .split_whitespace()
        .map(|row| row.split("->").collect())
        .filter(|elem: &String| !elem.is_empty())
        .map(|elem| elem.split(",").map(|s| s.to_string()).collect::<Vec<_>>())
        .map(|coords| (coords[0].parse::<u64>().unwrap(), coords[1].parse::<u64>().unwrap()))
        .collect();

    values
        .chunks(2)
        .map(|values| (make_coords(&values[0]), make_coords(&values[1])))
        .collect()
}

fn solve(input: Vec<(Coords, Coords)>) -> usize {
    let mut overlapping: HashMap<Coords, u64> = HashMap::new();

    for line in input {
        let (begin, end) = line;
        if begin.x == end.x {
            for y in min(begin.y, end.y)..max(begin.y, end.y) + 1 {
                *overlapping.entry(Coords { x: begin.x, y }).or_insert(0) += 1;
            }
        } else if begin.y == end.y {
            for x in min(begin.x, end.x)..max(begin.x, end.x) + 1 {
                *overlapping.entry(Coords { x, y: begin.y }).or_insert(0) += 1;
            }
        }
    }

    overlapping.values().filter(|times| **times > 1).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file provided");
        return;
    }

    let input = load_file(&args[1]);
    let result = solve(input);
    println!("{}", result);
}
