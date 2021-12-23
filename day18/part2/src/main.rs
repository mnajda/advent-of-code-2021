use std::env;
use std::fs;

#[derive(Debug)]
struct SnailfishNumber {
    values: Vec<u64>,
    depths: Vec<u8>,
}

impl SnailfishNumber {
    fn load_number(input: &String) -> SnailfishNumber {
        let mut number = SnailfishNumber {
            values: Vec::new(),
            depths: Vec::new(),
        };

        let mut depth = 0;
        for c in input.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => (),
                value => {
                    number
                        .values
                        .push(value.to_digit(10).unwrap().try_into().unwrap());
                    number.depths.push(depth - 1);
                }
            };
        }

        number
    }

    fn explode_value(&mut self, index: usize) {
        if index != 0 {
            self.values[index - 1] += self.values[index];
        }

        if index + 2 < self.values.len() {
            self.values[index + 2] += self.values[index + 1];
        }

        self.values[index] = 0;
        self.depths[index] = 3;
        self.values.remove(index + 1);
        self.depths.remove(index + 1);
    }

    fn explode(&mut self) -> bool {
        for i in 0..self.depths.len() {
            let depth = self.depths[i];
            if depth < 4 {
                continue;
            }

            self.explode_value(i);

            return true;
        }

        false
    }

    fn split_value(value: u64) -> (u64, u64) {
        if value % 2 == 0 {
            (value / 2, value / 2)
        } else {
            (value / 2, value / 2 + 1)
        }
    }

    fn split(&mut self) -> bool {
        for i in 0..self.values.len() {
            let value = self.values[i];
            if value < 10 {
                continue;
            }

            let (left, right) = SnailfishNumber::split_value(value);
            self.values[i] = left;
            self.values.insert(i + 1, right);

            self.depths[i] += 1;
            self.depths.insert(i + 1, self.depths[i]);

            return true;
        }

        false
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn add(&mut self, other: SnailfishNumber) {
        self.values.extend(other.values.iter());
        self.depths.extend(other.depths.iter());

        for depth in &mut self.depths {
            *depth += 1;
        }

        self.reduce();
    }

    fn magnitude(&mut self) -> u64 {
        while self.values.len() > 1 {
            for i in 0..self.depths.len() - 1 {
                if self.depths[i] == self.depths[i + 1] {
                    let left = self.values[i] * 3;
                    let right = self.values[i + 1] * 2;
                    self.values[i] = left + right;

                    self.values.remove(i + 1);
                    self.depths.remove(i + 1);

                    if self.depths[i] > 0 {
                        self.depths[i] -= 1
                    }

                    break;
                }
            }
        }

        self.values[0]
    }
}

fn load_file(path: &String) -> Vec<String> {
    let contents = fs::read_to_string(path).expect("Error reading file");

    contents.split("\n").map(|line| line.to_string()).collect()
}

fn add_numbers(lhs: &String, rhs: &String) -> u64 {
    let mut number = SnailfishNumber::load_number(lhs);
    let other = SnailfishNumber::load_number(rhs);

    number.add(other);
    number.magnitude()
}

fn find_max_magnitude(input: Vec<String>) -> u64 {
    let mut permutations: Vec<(String, String)> = Vec::new();
    for i in 0..input.len() {
        for j in 1..input.len() - 1 {
            permutations.push((input[i].clone(), input[j].clone()));
            permutations.push((input[j].clone(), input[i].clone()));
        }
    }

    permutations
        .into_iter()
        .flat_map(|elem| [add_numbers(&elem.0, &elem.1), add_numbers(&elem.1, &elem.0)])
        .max()
        .unwrap_or(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file provided");
        return;
    }

    let input = load_file(&args[1]);
    let result = find_max_magnitude(input);
    println!("{}", result);
}
