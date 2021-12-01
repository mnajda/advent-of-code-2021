use std::env;
use std::fs;

fn load_file(path: &String) -> Vec<u64> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    contents
        .split_whitespace()
        .map(|row| row.parse().unwrap())
        .collect()
}

fn solve(input: Vec<u64>) -> usize {
    input.windows(2).filter(|&pair| pair[0] < pair[1]).count()
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
