use std::collections::HashMap;
use std::env;
use std::fs;

fn load_file(path: &String) -> (String, HashMap<(char, char), char>) {
    let contents = fs::read_to_string(path).expect("Error reading file");
    let formula = contents.split("\n\n").collect::<Vec<_>>();

    let template = formula[0].to_string();
    let instructions = formula[1]
        .split("\n")
        .map(|row| row.split(" -> ").collect::<Vec<_>>())
        .map(|row| {
            (
                (
                    row[0].chars().next().unwrap(),
                    row[0].chars().skip(1).next().unwrap(),
                ),
                row[1].chars().next().unwrap(),
            )
        })
        .into_iter()
        .collect::<HashMap<_, _>>();

    (template, instructions)
}

fn initial_state(template: String) -> HashMap<(char, char), usize> {
    template.chars().collect::<Vec<_>>().windows(2).fold(
        HashMap::<(char, char), usize>::new(),
        |mut map, elem| {
            *map.entry((elem[0], elem[1])).or_default() += 1;
            map
        },
    )
}

fn step(
    occurences: HashMap<(char, char), usize>,
    instructions: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    occurences.into_iter().fold(
        HashMap::<(char, char), usize>::new(),
        |mut map, occurence| {
            *map.entry((occurence.0 .0, instructions[&occurence.0]))
                .or_default() += occurence.1;
            *map.entry((instructions[&occurence.0], occurence.0 .1))
                .or_default() += occurence.1;
            map
        },
    )
}

fn solve(input: (String, HashMap<(char, char), char>)) -> usize {
    let (template, instructions) = input;

    let mut state = initial_state(template);
    for _ in 0..40 {
        state = step(state, &instructions);
    }

    let occurences = state
        .into_iter()
        .fold(HashMap::<char, usize>::new(), |mut map, elem| {
            *map.entry(elem.0 .0).or_default() += elem.1;
            *map.entry(elem.0 .1).or_default() += elem.1;
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

    ((max - min) as f64 / 2.0) as usize
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
