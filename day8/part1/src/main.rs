use std::collections::HashSet;
use std::env;
use std::fs;

fn load_file(path: &String) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    contents
        .split("\n")
        .map(|row| row.split("|").skip(1).next().unwrap().trim().to_string())
        .map(|row| row.split_whitespace().map(|num| num.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
        .into_iter()
        .flatten()
        .collect()
}

fn is_unique(number: &String) -> bool {
    let unique: HashSet<char> = number.chars().collect();

    match unique.len() {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}

fn solve(input: Vec<String>) -> usize {
    input.iter().filter(|number| is_unique(*number)).count()
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
