use std::collections::HashMap;
use std::env;
use std::fs;

fn load_file(path: &String) -> (String, HashMap<String, char>) {
    let contents = fs::read_to_string(path).expect("Error reading file");
    let formula = contents.split("\n\n").collect::<Vec<_>>();

    let template = formula[0].to_string();
    let instructions = formula[1]
        .split("\n")
        .map(|row| row.split(" -> ").collect::<Vec<_>>())
        .map(|row| (row[0].to_string(), row[1].chars().next().unwrap()))
        .into_iter()
        .collect::<HashMap<_, _>>();

    (template, instructions)
}

fn insert_pairs(template: String, instructions: &HashMap<String, char>) -> String {
    let inserted = template
        .chars()
        .collect::<Vec<_>>()
        .windows(2)
        .map(|pair| {
            [
                pair[0],
                instructions[&String::from_iter(pair.iter())],
                pair[1],
            ]
        })
        .collect::<Vec<_>>();
    let mut new_template = Vec::from_iter(inserted[0]);
    new_template.append(
        &mut inserted[1..]
            .iter()
            .map(|triple| Vec::from_iter([triple[1], triple[2]]))
            .flatten()
            .collect::<Vec<_>>(),
    );

    String::from_iter(new_template)
}

fn solve(input: (String, HashMap<String, char>)) -> usize {
    let (mut template, instructions) = input;

    for _ in 0..10 {
        template = insert_pairs(template, &instructions);
    }

    let occurences =
        template
            .chars()
            .into_iter()
            .fold(HashMap::<char, usize>::new(), |mut map, elem| {
                *map.entry(elem).or_default() += 1;
                map
            });

    let max = occurences
        .iter()
        .max_by_key(|(_, times)| *times)
        .map(|(_, times)| *times)
        .unwrap();
    let min = occurences
        .iter()
        .min_by_key(|(_, times)| *times)
        .map(|(_, times)| *times)
        .unwrap();

    max - min
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
