use std::collections::HashSet;
use std::env;
use std::fs;

fn split_numbers(input: &String) -> Vec<String> {
    input
        .split_whitespace()
        .map(|number| number.to_string())
        .collect()
}

fn load_file(path: &String) -> Vec<(Vec<String>, Vec<String>)> {
    let contents = fs::read_to_string(path).expect("Error reading file");
    contents
        .split("\n")
        .map(|row| {
            row.split("|")
                .map(|part| part.to_string())
                .collect::<Vec<_>>()
        })
        .map(|row| (split_numbers(&row[0]), split_numbers(&row[1])))
        .collect()
}

fn is_unique(number: &String) -> bool {
    let unique: HashSet<char> = number.chars().collect();

    match unique.len() {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}

fn to_digit(signals: String) -> char {
    let mut chars = signals.chars().collect::<Vec<char>>();
    chars.sort();
    match String::from_iter(chars).as_str() {
        "abcefg" => '0',
        "cf" => '1',
        "acdeg" => '2',
        "acdfg" => '3',
        "bcdf" => '4',
        "abdfg" => '5',
        "abdefg" => '6',
        "acf" => '7',
        "abcdefg" => '8',
        "abcdfg" => '9',
        _ => unreachable!(),
    }
}

fn find_matching_length(signals: &Vec<String>, length: usize) -> String {
    signals
        .iter()
        .find(|signal| signal.len() == length)
        .unwrap()
        .to_string()
}

fn solve(input: (Vec<String>, Vec<String>)) -> u64 {
    let segments: HashSet<char> = HashSet::from_iter(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']);

    let (signals, digits) = input;
    let cf: HashSet<char> = HashSet::from_iter(find_matching_length(&signals, 2).chars());
    let acf: HashSet<char> = HashSet::from_iter(find_matching_length(&signals, 3).chars());
    let bcdf: HashSet<char> = HashSet::from_iter(find_matching_length(&signals, 4).chars());

    let still_unknown_segments: String = signals
        .iter()
        .filter(|signal| !is_unique(&signal))
        .map(|signal| signal.to_string())
        .fold("".to_string(), |acc, sig| acc + sig.as_str());

    let e = segments
        .iter()
        .filter(|segment| still_unknown_segments.matches(**segment).count() == 3)
        .map(|segment| segment.clone())
        .collect::<HashSet<char>>()
        .iter()
        .next()
        .unwrap()
        .clone();
    let bc: HashSet<char> = segments
        .iter()
        .filter(|segment| still_unknown_segments.matches(**segment).count() == 4)
        .map(|segment| segment.clone())
        .collect::<HashSet<char>>();

    let a = acf.difference(&cf).next().unwrap().clone();
    let b = bc.difference(&cf).next().unwrap().clone();
    let c = acf.intersection(&bc).next().unwrap().clone();
    let bd = bcdf
        .difference(&acf)
        .map(|c| c.clone())
        .collect::<HashSet<char>>();
    let d = bd
        .difference(&HashSet::from_iter(vec![b]))
        .next()
        .unwrap()
        .clone();
    let f = cf
        .difference(&HashSet::from_iter(vec![c]))
        .next()
        .unwrap()
        .clone();

    let decode = |segment: char| {
        if segment == a {
            'a'
        } else if segment == b {
            'b'
        } else if segment == c {
            'c'
        } else if segment == d {
            'd'
        } else if segment == e {
            'e'
        } else if segment == f {
            'f'
        } else {
            'g'
        }
    };

    digits
        .into_iter()
        .map(|digit| to_digit(digit.chars().map(|segment| decode(segment)).collect()))
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file provided");
        return;
    }

    let input = load_file(&args[1]);
    let result: u64 = input.into_iter().map(|entry| solve(entry)).sum();
    println!("{}", result);
}
