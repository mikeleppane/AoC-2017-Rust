use evalexpr::*;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[allow(dead_code)]
const TEST_INPUT: &str = include_str!("./test_input.txt");

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug)]
enum IntOperation {
    Increment,
    Decrement,
}

#[derive(Debug)]
struct Condition {
    register: String,
    operator: String,
    value: i32,
}

#[derive(Debug)]
struct Instruction {
    register: String,
    operation: IntOperation,
    amount: i32,
    condition: Condition,
}

fn read_input(input: &str) -> (Vec<Instruction>, HashMap<String, i32>) {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut registers: HashMap<String, i32> = HashMap::new();
    let re = Regex::new(r"^(\w+)\s(inc|dec)\s(-?\d{1,5})\sif\s(\w+)\s(.+)\s(-?\d{1,5})$").unwrap();
    for v in input.split('\n') {
        for cap in re.captures_iter(v) {
            registers.entry(cap[1].to_string()).or_insert(0);
            registers.entry(cap[4].to_string()).or_insert(0);
            instructions.push(Instruction {
                register: cap[1].to_string(),
                operation: match &cap[2] {
                    "inc" => IntOperation::Increment,
                    "dec" => IntOperation::Decrement,
                    _ => panic!("Should never be here"),
                },
                amount: i32::from_str(&cap[3]).unwrap(),
                condition: Condition {
                    register: cap[4].to_string(),
                    operator: cap[5].to_string(),
                    value: i32::from_str(&cap[6]).unwrap(),
                },
            })
        }
    }
    (instructions, registers)
}

pub fn run() {
    println!("=== Day 8 ===");
    part1(INPUT);
    part2(INPUT);
    println!("=============");
    println!();
}

fn find_largest_register_value(registers: &HashMap<String, i32>) -> i32 {
    *registers.values().max().unwrap()
}

fn part1(input: &str) -> i32 {
    let (instructions, mut registers) = read_input(input);
    for instruction in &instructions {
        let should_be_modified = eval_boolean(
            format!(
                "({}) {} ({})",
                registers[instruction.condition.register.as_str()],
                instruction.condition.operator.as_str(),
                instruction.condition.value
            )
            .as_str(),
        )
        .unwrap();
        if should_be_modified {
            let amount = match instruction.operation {
                IntOperation::Increment => {
                    registers[instruction.register.as_str()] + instruction.amount
                }
                IntOperation::Decrement => {
                    registers[instruction.register.as_str()] - instruction.amount
                }
            };

            registers.insert(instruction.register.to_owned(), amount);
        }
    }
    let largest_value = find_largest_register_value(&registers);
    println!("Day 8 part 1, solution: {}", largest_value);
    largest_value
}

fn part2(input: &str) -> i32 {
    let (instructions, mut registers) = read_input(input);
    let mut highest_value_of_all_time = i32::MIN;
    for instruction in &instructions {
        let should_be_modified = eval_boolean(
            format!(
                "({}) {} ({})",
                registers[instruction.condition.register.as_str()],
                instruction.condition.operator.as_str(),
                instruction.condition.value
            )
            .as_str(),
        )
        .unwrap();
        if should_be_modified {
            let amount = match instruction.operation {
                IntOperation::Increment => {
                    registers[instruction.register.as_str()] + instruction.amount
                }
                IntOperation::Decrement => {
                    registers[instruction.register.as_str()] - instruction.amount
                }
            };
            if amount > highest_value_of_all_time {
                highest_value_of_all_time = amount
            }
            registers.insert(instruction.register.to_owned(), amount);
        }
    }
    println!("Day 8 part 2, solution: {}", highest_value_of_all_time);
    highest_value_of_all_time
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_eq;

    #[test]
    fn part1_works_with_test_input() {
        assert_eq!(part1(TEST_INPUT), 1);
    }

    #[test]
    fn part1_works_with_puzzle_input() {
        assert_eq!(part1(INPUT), 4888);
    }

    #[test]
    fn part2_works_with_test_input() {
        assert_eq!(part2(TEST_INPUT), 10);
    }

    #[test]
    fn part2_works_with_puzzle_input() {
        assert_eq!(part2(INPUT), 7774);
    }
}
